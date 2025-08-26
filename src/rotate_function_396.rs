#[allow(dead_code)]
fn rotate_function(nums: Vec<i32>) -> i32 {
    // Not an algorithmic problem but a boring math problem.
    // Just figure out a math function that maps f(n) to f(n+1).
    // Then run this function for each n and find the max.
    todo!()
}

#[allow(dead_code)]
fn brute_force(mut nums: Vec<i32>) -> i32 {
    (0..nums.len())
        .map(|_| {
            nums.rotate_right(1);
            nums.iter()
                .enumerate()
                .fold(0, |acc, (i, &x)| acc + i as i32 * x)
        })
        .max()
        .expect("nums is empty")
}

#[cfg(test)]
mod tests {
    use crate::rotate_function_396::{brute_force, rotate_function};

    #[test]
    fn it_works() {
        assert_eq!(rotate_function(vec![4, 3, 2, 6]), 26);
        assert_eq!(rotate_function(vec![100]), 0);
        assert_eq!(
            brute_force(vec![2, 5, 8, 6, 12, 28, 2]),
            rotate_function(vec![2, 5, 8, 6, 12, 28, 2])
        );
        assert_eq!(
            brute_force(vec![2, -5, 8, 6, -12, 28, -2, -5]),
            rotate_function(vec![2, -5, 8, 6, -12, 28, -2, -5])
        );
    }
}
