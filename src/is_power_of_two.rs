pub fn is_power_of_two(num: u32) -> bool {
    // ex) 8
    // num     == 0b1000
    // num - 1 == 0b0111
    (num & (num - 1)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_power_of_two() {
        assert_eq!(true, is_power_of_two(8));
        assert_eq!(true, is_power_of_two(2 << 20));
        assert_eq!(false, is_power_of_two((2 << 20) + 1));
    }
}
