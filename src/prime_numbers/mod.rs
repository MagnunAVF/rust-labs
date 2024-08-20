pub fn using_while(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limit = (num as f64).sqrt() as u32;
    let mut div = 2;
    while div <= limit {
        if num % div == 0 {
            return false;
        }
        div += 1;
    }
    true
}

pub fn using_for(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limit = (num as f64).sqrt() as u32;
    for div in 2..=limit {
        if num % div == 0 {
            return false;
        }
    }

    true
}

// optimized version
pub fn using_for_v2(num: u32) -> bool {
    if num <= 1 {
        return false;
    } else if num == 2 || num == 3 {
        return true;
    } else if num % 2 == 0 || num % 3 == 0 {
        return false;
    } else {
        let limit = (num as f64).sqrt() as u32;
        for div in (5..=limit).step_by(2) {
            if num % div == 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::prime_numbers::{using_for, using_for_v2, using_while};

    #[test]
    fn test_using_while() {
        assert_eq!(using_while(1), false);
        assert_eq!(using_while(2), true);
        assert_eq!(using_while(3), true);
        assert_eq!(using_while(8), false);
        assert_eq!(using_while(97), true);
    }

    #[test]
    fn test_using_for() {
        assert_eq!(using_for(1), false);
        assert_eq!(using_for(2), true);
        assert_eq!(using_for(3), true);
        assert_eq!(using_for(8), false);
        assert_eq!(using_for(97), true);
    }

    #[test]
    fn test_using_for_v2() {
        assert_eq!(using_for_v2(1), false);
        assert_eq!(using_for_v2(2), true);
        assert_eq!(using_for_v2(3), true);
        assert_eq!(using_for_v2(8), false);
        assert_eq!(using_for_v2(97), true);
    }
}
