// pub fn is_string_palindrome(s: &str) -> bool {
//     fn aux(s: &str, curr: usize) -> bool {
//         let opp = (s.len() - 1) - curr;
//         if curr >= opp {
//             return true;
//         }

//         match (s.get(curr..=curr), s.get(opp..=opp)) {
//             (Some(f), Some(l)) if f == l => aux(s, curr + 1),
//             _ => false,
//         }
//     }

//     s.len() < 1 || aux(s, 0)
// }

pub fn is_string_palindrome(s: &str) -> bool {
    fn aux(s: &str, curr: usize, len: usize) -> bool {
        if curr >= len / 2 {
            return true;
        }

        let opp = len - 1 - curr;

        match (s.get(curr..=curr), s.get(opp..=opp)) {
            (Some(f), Some(l)) if f == l => aux(s, curr + 1, len),
            _ => false,
        }
    }

    aux(s, 0, s.len())
}
