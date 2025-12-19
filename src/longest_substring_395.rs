use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn longest_substring(s: String, k: i32) -> i32 {
    let mut char_freq: HashMap<_, i32> = HashMap::new();
    for c in s.chars() {
        char_freq.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    let too_rare_chars = char_freq
        .iter()
        .filter(|(_, v)| *v < &k)
        .map(|(k, _)| *k)
        .collect::<HashSet<_>>();
    if too_rare_chars.is_empty() {
        s.len() as i32
    } else {
        s
            .split(|it| too_rare_chars.contains(&it))
            .filter(|it| it.len() >= k as usize)
            .map(|it| longest_substring(it.to_string(), k))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_substring_395::longest_substring;

    #[test]
    fn test1_395() {
        assert_eq!(longest_substring("ababbc".into(), 2), 5);
        assert_eq!(longest_substring("aaabb".into(), 3), 3);
        assert_eq!(longest_substring("abcabcbb".into(), 3), 0);
        assert_eq!(longest_substring("dabcabcbb".into(), 2), 8);
        assert_eq!(longest_substring("dabbaebcbb".into(), 2), 4);
        assert_eq!(longest_substring("bbbbb".into(), 1), 5);
        assert_eq!(longest_substring("pwwkew".into(), 2), 2);
    }
}
