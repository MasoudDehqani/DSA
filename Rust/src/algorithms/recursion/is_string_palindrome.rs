pub fn is_string_palindrome(s: &str) -> bool {
    if s.len() < 1 {
        return true;
    }

    fn aux(s: &str, curr: usize) -> bool {
        let opp = (s.len() - 1) - curr;
        if curr >= opp {
            return true;
        }

        match (s.get(0..=curr), s.get(opp..)) {
            (Some(f), Some(l)) if f.chars().last() == l.chars().next() => aux(s, curr + 1),
            _ => false,
        }
    }

    aux(s, 0)
}
