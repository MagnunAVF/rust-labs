pub fn classic(n: u64) -> u64 {
    let mut result = 1;

    for i in 2..=n {
        result *= i;
    }

    result
}

// not memory efficient
pub fn recursive(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * recursive(n - 1)
    }
}

pub fn using_iterator(n: u64) -> u64 {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use crate::factorial::{classic, recursive, using_iterator};

    #[test]
    fn test_classic_factorial() {
        assert_eq!(classic(0), 1);
        assert_eq!(classic(1), 1);
        assert_eq!(classic(5), 120);
        assert_eq!(classic(10), 3628800);
    }

    #[test]
    fn test_recursive_factorial() {
        assert_eq!(recursive(0), 1);
        assert_eq!(recursive(1), 1);
        assert_eq!(recursive(5), 120);
        assert_eq!(recursive(10), 3628800);
    }

    #[test]
    fn test_iterator_factorial() {
        assert_eq!(using_iterator(0), 1);
        assert_eq!(using_iterator(1), 1);
        assert_eq!(using_iterator(5), 120);
        assert_eq!(using_iterator(10), 3628800);
    }

    #[test]
    #[should_panic]
    fn test_overflow() {
        classic(21);
    }
}
