#[allow(dead_code)]
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut x = x as u64;
    let mut items = vec![];
    while x > 0 {
        items.push(x % 10);
        x /= 10;
    }
    for i in 0..items.len() / 2 {
        if items[i] != items[items.len() - i - 1] {
            return false;
        }
    }
    true
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(1221), true);
    assert_eq!(is_palindrome(1234554321), true);
    assert_eq!(is_palindrome(123454321), true);
    assert_eq!(is_palindrome(12345321), false);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(0), true);
    assert_eq!(is_palindrome(8), true);
    assert_eq!(is_palindrome(2147483647), false);
    assert_eq!(is_palindrome(-2147483648), false);
}
