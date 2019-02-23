pub fn factorial(num: u32) -> u32 {
    match num {
        1 => 1,
        _ => num * factorial(num - 1),
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;
    #[test]
    fn test_factorial() {
        let test_data: std::collections::HashMap<u32, u32> = [
            (1, 1),
            (2, 2),
            (3, 6),
            (4, 24),
            (5, 120),
            (6, 720),
            (7, 5040),
            (8, 40320),
            (9, 362880),
            (10, 3628800),
        ]
        .iter()
        .cloned()
        .collect();

        for (input, expected) in &test_data {
            assert_eq!(*expected, factorial::factorial(*input));
        }
    }
}
