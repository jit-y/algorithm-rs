pub fn fib(num: u32) -> u32 {
    match num {
        0 => num,
        1 => num,
        _ => fib(num - 1) + fib(num - 2),
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;
    #[test]
    fn test_fib() {
        assert_eq!(0, fibonacci::fib(0));
        assert_eq!(1, fibonacci::fib(1));
        assert_eq!(1, fibonacci::fib(2));
        assert_eq!(2, fibonacci::fib(3));
        assert_eq!(3, fibonacci::fib(4));
    }
}
