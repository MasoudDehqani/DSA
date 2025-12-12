pub fn reverse_string(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();
    let rest = chars.as_str().to_string();

    reverse_string(rest) + &first.to_string()
}
