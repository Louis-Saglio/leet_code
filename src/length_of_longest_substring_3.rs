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
    let mut i = 0;
    let chars = s.as_bytes();
    let mut max_len = 0;
    let mut current_len = 0;
    let mut index_by_char = HashMap::new();
    while i != s.len() {
        let mut char = chars[i];
        if index_by_char.contains_key(&char) {
            i = index_by_char[&char] + 1;
            char = chars[i];
            current_len = 0;
            index_by_char.clear();
        }
        index_by_char.insert(char, i);
        current_len += 1;
        max_len = max(max_len, current_len);
        i += 1;
    }
    max_len
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
