pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use crate::gcd;
    #[test]
    fn test_gcd() {
        assert_eq!(21, gcd::gcd(252, 105));
    }
}
