/**
Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

    '.' Matches any single character.
    '*' Matches zero or more of the preceding element.

The matching should cover the entire input string (not partial).



Example 1:

Input: s = "aa", p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".

Example 2:

Input: s = "aa", p = "a*"
Output: true
Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".

Example 3:

Input: s = "ab", p = ".*"
Output: true
Explanation: ".*" means "zero or more (*) of any character (.)".



Constraints:

    1 <= s.length <= 20
    1 <= p.length <= 20
    s contains only lowercase English letters.
    p contains only lowercase English letters, '.', and '*'.
    It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.

**/

struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_match(s: String, p: String) -> bool {
        let string = s.into_bytes();
        let pattern = p.into_bytes();
        Self::is_match_(&string, &pattern)
    }

    fn is_match_(string: &[u8], pattern: &[u8]) -> bool {
        if pattern.is_empty() {
            string.is_empty()
        } else if pattern.get(1) == Some(&b'*') {
            let mut greed = 0;
            while greed <= string.len()
                && (greed == 0 || string[greed - 1] == pattern[0] || pattern[0] == b'.')
            {
                if Self::is_match_(&string[greed..], &pattern[2..]) {
                    return true;
                }
                greed += 1;
            }
            false
        } else if string.is_empty() {
            false
        } else if pattern[0] == b'.' {
            Self::is_match_(&string[1..], &pattern[1..])
        } else {
            if pattern[0] == string[0] {
                Self::is_match_(&string[1..], &pattern[1..])
            } else {
                false
            }
        }
    }
}

#[test]
fn test_0() {
    assert_eq!(Solution::is_match("aa".to_string(), "aa".to_string()), true);
}

#[test]
fn test_1() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
}

#[test]
fn test_2() {
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
}

#[test]
fn test_3() {
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
}

#[test]
fn test_4() {
    assert_eq!(
        Solution::is_match("aaaaab".to_string(), "aa*b".to_string()),
        true
    );
}

#[test]
fn test_5() {
    assert_eq!(
        Solution::is_match("aab".to_string(), "aa*b".to_string()),
        true
    );
}

#[test]
fn test_6() {
    assert_eq!(
        Solution::is_match("aab".to_string(), "c*a*b".to_string()),
        true
    )
}

#[test]
fn test_7() {
    assert_eq!(
        Solution::is_match("aab".to_string(), "caabd".to_string()),
        false
    )
}

#[test]
fn test_8() {
    assert_eq!(Solution::is_match("a".to_string(), "a".to_string()), true)
}

#[test]
fn test_9() {
    assert_eq!(
        Solution::is_match("abcd".to_string(), "d*".to_string()),
        false
    )
}

#[test]
fn test_10() {
    assert_eq!(
        Solution::is_match("aab".to_string(), "c*".to_string()),
        false
    )
}

#[test]
fn test_11() {
    assert_eq!(
        Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
        false
    )
}

#[test]
fn test_12() {
    assert_eq!(
        Solution::is_match("aaa".to_string(), "a*a".to_string()),
        true
    )
}

#[test]
fn test_13() {
    assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true)
}
