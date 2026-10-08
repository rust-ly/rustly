fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    println!("{}", greet("Ada"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_ada() {
        assert_eq!(greet("Ada"), "Hello, Ada!");
    }

    #[test]
    fn greets_anyone() {
        assert_eq!(greet("Ferris"), "Hello, Ferris!");
    }
}
