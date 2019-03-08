pub fn pascal_triangle(line_number: u32) -> Vec<u32> {
    if line_number == 0 {
        return vec![1];
    }

    let current_line_size = line_number as usize + 1;
    let previous_line_size = current_line_size - 1;

    let mut current_line = vec![0; current_line_size];
    let previous_line = pascal_triangle(line_number - 1);

    for i in 0..current_line_size {
        let left_coefficient = if i > 0 { previous_line[i - 1] } else { 0 };
        let right_coefficient = if i < previous_line_size {
            previous_line[i]
        } else {
            0
        };

        current_line[i] = left_coefficient + right_coefficient;
    }

    current_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_triangle() {
        assert_eq!(vec![1], pascal_triangle(0));
        assert_eq!(vec![1, 1], pascal_triangle(1));
        assert_eq!(vec![1, 2, 1], pascal_triangle(2));
        assert_eq!(vec![1, 3, 3, 1], pascal_triangle(3));
        assert_eq!(vec![1, 4, 6, 4, 1], pascal_triangle(4));
    }
}
