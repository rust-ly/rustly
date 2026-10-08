//! A stand-in for the Playground, so endpoint tests run without a network.

// Each test binary uses a different part of this module.
#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use runner::PlaygroundClient;
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// A server that answers every request with the same status and body, and
/// records the JSON it was sent.
pub struct FakePlayground {
    pub url: String,
    received: Arc<Mutex<Vec<(String, Value)>>>,
}

impl FakePlayground {
    /// Answers `200` with a Playground-shaped body.
    pub async fn answering(success: bool, stdout: &str, stderr: &str) -> Self {
        let body = json!({
            "success": success,
            "exitDetail": "",
            "stdout": stdout,
            "stderr": stderr,
        });
        Self::with_status(200, body.to_string()).await
    }

    pub async fn with_status(status: u16, body: String) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let received = Arc::new(Mutex::new(Vec::new()));
        let log = received.clone();
        tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                serve(stream, status, &body, &log).await;
            }
        });
        Self { url, received }
    }

    pub fn client(&self) -> PlaygroundClient {
        PlaygroundClient::new(&self.url)
    }

    /// `(path, body)` of every request so far.
    pub fn received(&self) -> Vec<(String, Value)> {
        self.received.lock().unwrap().clone()
    }
}

async fn serve(mut stream: TcpStream, status: u16, body: &str, log: &Mutex<Vec<(String, Value)>>) {
    let mut buf = Vec::new();
    let mut chunk = [0; 4096];
    let header_end = loop {
        let n = stream.read(&mut chunk).await.unwrap();
        buf.extend_from_slice(&chunk[..n]);
        if let Some(i) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            break i + 4;
        }
    };
    let head = String::from_utf8_lossy(&buf[..header_end]).to_string();
    let length: usize = head
        .lines()
        .find_map(|l| {
            let (name, value) = l.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse().unwrap())
        })
        .unwrap_or(0);
    while buf.len() < header_end + length {
        let n = stream.read(&mut chunk).await.unwrap();
        buf.extend_from_slice(&chunk[..n]);
    }
    let path = head.split_whitespace().nth(1).unwrap_or("").to_string();
    let sent = serde_json::from_slice(&buf[header_end..header_end + length]).unwrap_or(Value::Null);
    log.lock().unwrap().push((path, sent));

    let response = format!(
        "HTTP/1.1 {status} X\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes()).await.unwrap();
}
