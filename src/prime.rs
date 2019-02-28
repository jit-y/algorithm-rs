pub fn is_prime_number(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    if num <= 3 {
        return true;
    }

    if num % 2 == 0 {
        return false;
    }

    let limit = (num as f32).sqrt() as u32;

    for i in (3..=limit).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::prime;

    #[test]
    fn test_is_prime_number() {
        assert_eq!(false, prime::is_prime_number(1));
        assert_eq!(true, prime::is_prime_number(2));
        assert_eq!(true, prime::is_prime_number(3));
        assert_eq!(true, prime::is_prime_number(997));
    }
}
