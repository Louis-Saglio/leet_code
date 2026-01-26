use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> String {
    let mut map = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        map.entry(c).or_insert(Vec::new()).push(i);
    }
    for (c, indexes) in map {
        
    }
    todo!()
}

#[test]
fn test() {
    assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
}