use std::cmp::min;
use std::collections::HashMap;

#[allow(dead_code)]
fn integer_replacement_internal(n: u32, number_of_steps: u8, memory: &mut HashMap<u32, u8>) -> u8 {
    if let Some(result) = memory.get(&n) {
        return *result + number_of_steps;
    }
    if n == 1 {
        number_of_steps
    } else {
        let result = if n % 2 == 0 {
            integer_replacement_internal(n / 2, number_of_steps + 1, memory)
        } else {
            min(
                integer_replacement_internal(n + 1, number_of_steps + 1, memory),
                integer_replacement_internal(n - 1, number_of_steps + 1, memory),
            )
        };
        memory.insert(n, result - number_of_steps);
        result
    }
}

#[allow(dead_code)]
fn integer_replacement(n: i32) -> i32 {
    integer_replacement_internal(n as u32, 0, &mut HashMap::new()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    #[test]
    fn test_integer_replacement_0() {
        assert_eq!(integer_replacement(8), 3);
    }
    #[test]
    fn test_integer_replacement_1() {
        assert_eq!(integer_replacement(7), 4);
    }
    #[test]
    fn test_integer_replacement_2() {
        assert_eq!(integer_replacement(13), 5);
    }
    #[test]
    fn test_integer_replacement_3() {
        assert_eq!(integer_replacement(i32::MAX - 1), 33);
    }

    #[test]
    fn test_integer_replacement_4() {
        for i in 1..100 {
            integer_replacement(i);
        }
    }

    #[test]
    fn test_integer_replacement_5() {
        integer_replacement(37);
    }

    #[test]
    fn test_integer_replacement_6() {
        let start = Instant::now();
        for i in 1..100_000 {
            integer_replacement(i32::MAX - i);
        }
        println!("{:?}", start.elapsed());
    }
}

/*
13 14  7  8  4  2  1
13 12  6  3  4  2  1
13 14  7  6  3  2  1
13 12  6  3  2  1

25 24 12  6  3  2  1
25 26 13
*/
