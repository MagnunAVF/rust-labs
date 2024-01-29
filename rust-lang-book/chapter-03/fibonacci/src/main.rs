use std::io;

fn fibonacci(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    println!("### Fibonacci ###");

    println!("\nInput the n value:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the value");

    match input.trim().parse::<u128>() {
        Ok(n) => {
            let result = fibonacci(n);
            print!("The Nth number is {}.\n", result);
        }
        Err(_err) => print!("Invalid input!\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_0() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn test_fibonacci_1() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_9() {
        assert_eq!(fibonacci(9), 34);
    }

    #[test]
    fn test_fibonacci_10() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_fibonacci_18() {
        assert_eq!(fibonacci(18), 2584);
    }
}
