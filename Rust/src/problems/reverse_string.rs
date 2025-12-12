pub fn reverse_string(s: String) -> String {
    let mut res = String::new();

    for ch in s.chars() {
        res = ch.to_string() + &res;
    }

    res
}

pub fn reverse_string_recursive(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();
    let rest = chars.as_str().to_string();

    reverse_string_recursive(rest) + &first.to_string()
}

#[cfg(test)]
mod tests {
    use super::reverse_string;

    #[test]
    fn empty_string() {
        assert_eq!(reverse_string(String::from("")), String::from(""))
    }

    #[test]
    fn single_char_string() {
        assert_eq!(reverse_string(String::from("a")), String::from("a"))
    }

    #[test]
    fn simple_ascii() {
        assert_eq!(reverse_string(String::from("Hello")), String::from("olleH"))
    }

    #[test]
    fn palindrome() {
        assert_eq!(
            reverse_string(String::from("racecar")),
            String::from("racecar")
        )
    }

    #[test]
    fn repeated_chars() {
        assert_eq!(reverse_string("aaabbb".to_string()), "bbbaaa");
    }

    #[test]
    fn unicode_single_graphemes() {
        // Each emoji is a single Unicode scalar
        assert_eq!(reverse_string("🙂🙃".to_string()), "🙃🙂");
    }

    #[test]
    fn unicode_multi_byte() {
        // Persian, Arabic, etc. are multi-byte UTF-8 chars
        assert_eq!(reverse_string("سلام".to_string()), "مالس");
    }

    #[test]
    fn mixed_unicode_and_ascii() {
        assert_eq!(reverse_string("a🙂b".to_string()), "b🙂a");
    }

    #[test]
    fn whitespace() {
        assert_eq!(reverse_string("a b c".to_string()), "c b a");
    }

    #[test]
    fn long_string() {
        let s = "abcdefghijklmnopqrstuvwxyz".repeat(10);
        let r: String = s.chars().rev().collect();
        assert_eq!(reverse_string(s.clone()), r);
    }
}
