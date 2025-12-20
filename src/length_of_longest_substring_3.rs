/**
3. Longest Substring Without Repeating Characters

Given a string s, find the length of the longest substring without duplicate characters.

Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3. Note that "bca" and "cab" are also correct answers.

Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.



Constraints:

    0 <= s.length <= 5 * 104
    s consists of English letters, digits, symbols, and spaces.

**/
use std::cmp::max;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut last_seen_index_by_char = HashMap::new();
    let mut start = 0;
    let mut max_len = 0;
    for (end, char) in s.as_bytes().iter().enumerate() {
        if let Some(&last_seen_index) = last_seen_index_by_char.get(&char) {
            start = max(start, last_seen_index + 1);
        }
        last_seen_index_by_char.insert(char, end);
        max_len = max(max_len, end - start + 1);
    }
    max_len as i32
}

#[test]
pub fn test_1() {
    assert_eq!(length_of_longest_substring("a".into()), 1);
}

#[test]
pub fn test_2() {
    assert_eq!(length_of_longest_substring("abcde".into()), 5);
}

#[test]
pub fn test_3() {
    assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
}

#[test]
pub fn test_4() {
    assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
}

#[test]
pub fn test_5() {
    assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
}

#[test]
pub fn test_6() {
    assert_eq!(length_of_longest_substring("dvdf".into()), 3);
}

#[test]
pub fn test_7() {
    assert_eq!(length_of_longest_substring("".into()), 0);
}

#[test]
pub fn test_8() {
    assert_eq!(length_of_longest_substring("axebcrtudbefg".into()), 9);
}
