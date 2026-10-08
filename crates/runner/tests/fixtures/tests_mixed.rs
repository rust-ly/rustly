fn add(a: i32, b: i32) -> i32 {
    a + b + if a > 10 { 1 } else { 0 }
}

fn main() {
    println!("{}", add(2, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_small_numbers() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn adds_big_numbers() {
        assert_eq!(add(20, 22), 42);
    }

    #[test]
    fn panics_inside() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(v[0], 0);
    }
}
