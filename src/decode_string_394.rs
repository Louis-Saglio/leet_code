#[allow(dead_code)]
pub fn decode_string_394(s: String) -> String {
    let mut stack = Vec::new();
    let mut is_scanning_number = false;
    for c in format!("1[{}]", s).chars() {
        if c.is_digit(10) {
            if !is_scanning_number {
                is_scanning_number = true;
                stack.push((String::new(), String::new()));
            }
            stack.last_mut().unwrap().0.push(c);
        } else if c == '[' {
            is_scanning_number = false;
        } else if c == ']' {
            let (nbr, letters) = stack.pop().unwrap();
            let letters = letters.repeat(nbr.parse::<usize>().unwrap());
            match stack.last_mut() {
                None => return letters,
                Some((_, last_letters)) => last_letters.push_str(&letters),
            }
        } else {
            stack.last_mut().unwrap().1.push(c);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            decode_string_394("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        );
        assert_eq!(
            decode_string_394("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
        assert_eq!(
            decode_string_394("3[a2[c]]".to_string()),
            "accaccacc".to_string()
        );
        assert_eq!(
            decode_string_394("ab3[a2[c]]2[abc]3[cd]ef".to_string()),
            "abaccaccaccabcabccdcdcdef".to_string()
        );
        assert_eq!(
            decode_string_394("ab2[a10[c]]2[abc]3[cd]ef".to_string()),
            "ab acccccccccc acccccccccc abc abc cd cd cd ef"
                .replace(" ", "")
                .to_string()
        );
        assert_eq!(
            decode_string_394("2[a3[b12[c]]]".to_string()),
            "a bcccccccccccc bcccccccccccc bcccccccccccc a bcccccccccccc bcccccccccccc bcccccccccccc".replace(" ", "").to_string()
        );
        assert_eq!(
            decode_string_394("1[1[1[1[1[1[1[1[a]]]]]]]]".to_string()),
            "a".to_string()
        )
    }
}
