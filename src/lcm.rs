use super::gcd;

pub fn lcm(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }

    (a * b) / gcd::gcd(a, b)
}

#[cfg(test)]
mod tests {
    use crate::lcm;

    #[test]
    fn test_lcm() {
        assert_eq!(12, lcm::lcm(4, 6));
        assert_eq!(0, lcm::lcm(10, 0));
    }
}
