pub fn fib(num: u32) -> u32 {
    match num {
        0 => num,
        1 => num,
        _ => fib(num - 1) + fib(num - 2),
    }
}

pub fn fib_to_vec(num: i32) -> Vec<u32> {
    let mut vec = Vec::with_capacity((num + 1) as usize);

    for val in 0..=num {
        vec.push(fib(val as u32));
    }

    vec
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

    #[test]
    fn test_fib_to_vec() {
        assert_eq!(vec![0, 1, 1, 2, 3], fibonacci::fib_to_vec(4));
    }
}
