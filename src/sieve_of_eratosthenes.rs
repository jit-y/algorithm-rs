pub fn sieve_of_eratosthenes(max_num: u32) -> Vec<u32> {
    let mut is_primes = vec![true; max_num as usize + 1];
    let mut primes = Vec::new();

    is_primes[0] = false;
    is_primes[1] = true;

    for number in 2..=max_num {
        if is_primes[number as usize] == true {
            primes.push(number);

            let mut next_number = number * number;

            while next_number <= max_num {
                is_primes[next_number as usize] = false;
                next_number += number;
            }
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sieve_of_eratosthenes() {
        assert_eq!(
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
            sieve_of_eratosthenes(30)
        );
    }
}
