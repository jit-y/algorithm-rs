pub fn get_bit(num: i32, position: i32) -> i32 {
    (num >> position) & 1
}

pub fn set_bit(num: i32, position: i32) -> i32 {
    num | (1 << position)
}

pub fn clear_bit(num: i32, position: i32) -> i32 {
    let mask = !(1 << position);

    num & mask
}

#[cfg(test)]
mod tests {
    use crate::bit;
    #[test]
    fn test_get_bit() {
        assert_eq!(bit::get_bit(10, 0), 0);
        assert_eq!(bit::get_bit(10, 1), 1);
        assert_eq!(bit::get_bit(10, 2), 0);
        assert_eq!(bit::get_bit(10, 3), 1);
    }

    #[test]
    fn test_set_bit() {
        assert_eq!(bit::set_bit(10, 0), 11);
        assert_eq!(bit::set_bit(10, 1), 10);
        assert_eq!(bit::set_bit(10, 2), 14);
        assert_eq!(bit::set_bit(10, 3), 10);
        assert_eq!(bit::set_bit(10, 4), 26);
        assert_eq!(bit::set_bit(10, 5), 42);
    }

    #[test]
    fn test_clear_bit() {
        assert_eq!(bit::clear_bit(10, 0), 10);
        assert_eq!(bit::clear_bit(10, 1), 8);
        assert_eq!(bit::clear_bit(10, 2), 10);
        assert_eq!(bit::clear_bit(10, 3), 2);
        assert_eq!(bit::clear_bit(10, 4), 10);
    }
}
