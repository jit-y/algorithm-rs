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

pub fn update_bit(num: i32, position: i32, bit_value: bool) -> i32 {
    let normalized_bit_value: i32 = match bit_value {
        true => 1,
        false => 0,
    };

    let mask = !(1 << position);

    (num & mask) | (normalized_bit_value << position)
}

#[cfg(test)]
mod tests {
    use crate::bit;
    #[test]
    fn test_get_bit() {
        assert_eq!(0, bit::get_bit(10, 0));
        assert_eq!(1, bit::get_bit(10, 1));
        assert_eq!(0, bit::get_bit(10, 2));
        assert_eq!(1, bit::get_bit(10, 3));
    }

    #[test]
    fn test_set_bit() {
        assert_eq!(11, bit::set_bit(10, 0));
        assert_eq!(10, bit::set_bit(10, 1));
        assert_eq!(14, bit::set_bit(10, 2));
        assert_eq!(10, bit::set_bit(10, 3));
        assert_eq!(26, bit::set_bit(10, 4));
        assert_eq!(42, bit::set_bit(10, 5));
    }

    #[test]
    fn test_clear_bit() {
        assert_eq!(10, bit::clear_bit(10, 0));
        assert_eq!(8, bit::clear_bit(10, 1));
        assert_eq!(10, bit::clear_bit(10, 2));
        assert_eq!(2, bit::clear_bit(10, 3));
        assert_eq!(10, bit::clear_bit(10, 4));
    }

    #[test]
    fn test_update_bit() {
        assert_eq!(10, bit::update_bit(10, 1, true));
        assert_eq!(8, bit::update_bit(10, 1, false));
        assert_eq!(10, bit::update_bit(10, 3, true));
        assert_eq!(2, bit::update_bit(10, 3, false));
    }
}
