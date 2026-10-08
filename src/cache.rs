//! A small in-memory cache of compile-service results.
//!
//! Vercel reuses warm function instances, so the same program sent twice
//! (a learner pressing Run again, or several learners on the same starter)
//! skips the Playground the second time. It's best-effort: a cold start
//! begins empty.

use std::collections::HashMap;
use std::sync::Mutex;

use runner::playground::Output;

/// Most results kept. Programs are capped at 20 KB plus hidden tests, so this
/// stays around a few MB at worst.
const CAPACITY: usize = 64;

#[derive(Debug, Default)]
pub struct Cache {
    inner: Mutex<Inner>,
}

#[derive(Debug, Default)]
struct Inner {
    /// `(endpoint, program)` → result and when it was last used.
    entries: HashMap<(&'static str, String), (Output, u64)>,
    clock: u64,
}

impl Cache {
    pub fn get(&self, endpoint: &'static str, program: &str) -> Option<Output> {
        let mut inner = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        inner.clock += 1;
        let now = inner.clock;
        let (output, used) = inner.entries.get_mut(&(endpoint, program.to_string()))?;
        *used = now;
        Some(output.clone())
    }

    pub fn put(&self, endpoint: &'static str, program: &str, output: Output) {
        let mut inner = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        inner.clock += 1;
        let now = inner.clock;
        if inner.entries.len() >= CAPACITY {
            let oldest = inner
                .entries
                .iter()
                .min_by_key(|(_, (_, used))| *used)
                .map(|(key, _)| key.clone());
            if let Some(key) = oldest {
                inner.entries.remove(&key);
            }
        }
        inner
            .entries
            .insert((endpoint, program.to_string()), (output, now));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn output(stdout: &str) -> Output {
        Output {
            success: true,
            exit_detail: String::new(),
            stdout: stdout.into(),
            stderr: String::new(),
        }
    }

    #[test]
    fn returns_what_was_put() {
        let cache = Cache::default();
        cache.put("execute", "fn main() {}", output("hi"));
        assert_eq!(cache.get("execute", "fn main() {}"), Some(output("hi")));
    }

    #[test]
    fn keys_on_endpoint_and_program() {
        let cache = Cache::default();
        cache.put("execute", "fn main() {}", output("hi"));
        assert_eq!(cache.get("clippy", "fn main() {}"), None);
        assert_eq!(cache.get("execute", "fn main() { }"), None);
    }

    #[test]
    fn evicts_the_least_recently_used() {
        let cache = Cache::default();
        for i in 0..CAPACITY {
            cache.put("execute", &i.to_string(), output(""));
        }
        // Touch 0 so 1 becomes the oldest.
        cache.get("execute", "0");
        cache.put("execute", "new", output(""));
        assert!(cache.get("execute", "0").is_some());
        assert!(cache.get("execute", "1").is_none());
        assert!(cache.get("execute", "new").is_some());
    }
}
