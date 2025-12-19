use std::cmp::max;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    println!("\n{s}");
    let mut i = 0_usize;
    let mut max_len = 0;
    let mut current_len = 0;
    let chars = s.as_bytes();
    println!("{chars:?}");
    let mut index_by_char: HashMap<u8, usize> = HashMap::new();
    index_by_char.insert(chars[0], 0);
    while i != chars.len() {
        let char = chars[i];
        println!("{i}: {char}");
        if index_by_char.contains_key(&char) {
            i = *index_by_char.get(&char).unwrap();
            index_by_char.clear();
            index_by_char.insert(chars[i], i);
            current_len = 0;
        }
        index_by_char.insert(char, i);
        i += 1;
        current_len += 1;
        max_len = max(max_len, current_len);
    }
    max_len
}

#[test]
pub fn test() {
    // assert_eq!(length_of_longest_substring("a".into()), 1);
    // assert_eq!(length_of_longest_substring("abcde".into()), 5);
    // assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
    // assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
    // assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
    // assert_eq!(length_of_longest_substring("".into()), 0);
    assert_eq!(length_of_longest_substring("dvdf".into()), 3);
}
