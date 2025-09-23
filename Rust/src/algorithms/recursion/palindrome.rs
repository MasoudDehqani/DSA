// pub fn is_palidrome_string(s: String) -> bool {
//     let s1: String = s
//         .chars()
//         .filter(|ch| ch.is_alphanumeric())
//         .map(|ch| ch.to_ascii_lowercase())
//         .rev()
//         .collect();
//     let s2: String = s
//         .chars()
//         .filter(|ch| ch.is_alphanumeric())
//         .map(|ch| ch.to_ascii_lowercase())
//         .collect();

//     s1 == s2
// }

// pub fn is_palindrome_string(s: String) -> bool {
//     fn helper(str: String, start: usize, end: usize) -> bool {
//         if start >= end {
//             return true;
//         }

//         let clone_1 = str.clone();
//         let clone_2 = str.clone();
//         let mut chars_1 = clone_1.chars();
//         let mut chars_2 = clone_2.chars();
//         let maybe_start_char = chars_1.nth(start);
//         let maybe_end_char = chars_2.nth(end);

//         let is_start_valid = maybe_start_char.map_or(false, |ch| ch.is_alphanumeric());
//         let is_end_valid = maybe_end_char.map_or(false, |ch| ch.is_alphanumeric());

//         match (is_start_valid, is_end_valid) {
//             (true, true) => {
//                 if maybe_start_char.map(|ch| ch.to_ascii_lowercase())
//                     == maybe_end_char.map(|ch| ch.to_ascii_lowercase())
//                 {
//                     helper(str, start + 1, end - 1)
//                 } else {
//                     false
//                 }
//             }
//             (false, false) => helper(str, start + 1, end - 1),
//             (true, false) => helper(str, start, end - 1),
//             (false, true) => helper(str, start + 1, end),
//         }
//     }

//     match s.is_empty() || s.chars().count() == 1 {
//         true => true,
//         false => helper(s.clone(), 0, s.chars().count() - 1),
//     }
// }

pub fn is_palindrome_string(s: String) -> bool {
    fn helper(
        mut chars_iterator: impl Iterator<Item = char>,
        mut rev_chars_iterator: impl Iterator<Item = char>,
    ) -> bool {
        let first = chars_iterator.nth(0);
        let last = rev_chars_iterator.nth(0);

        if first.is_none() || last.is_none() {
            return true;
        }

        if first == last {
            helper(chars_iterator, rev_chars_iterator)
        } else {
            false
        }
    }

    let chars = s
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase());

    helper(chars.clone(), chars.rev())
}

// pub fn is_palindrome_string(s: String) -> bool {
//     for (f, s) in s
//         .chars()
//         .filter(|ch| ch.is_alphanumeric())
//         .map(|ch| ch.to_ascii_lowercase())
//         .zip(
//             s.chars()
//                 .filter(|ch| ch.is_alphanumeric())
//                 .map(|ch| ch.to_ascii_lowercase())
//                 .rev(),
//         )
//     {
//         if f == s {
//             continue;
//         } else {
//             return false;
//         }
//     }

//     true
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_string() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert!(is_palindrome_string(s));
    }

    #[test]
    fn non_palindrome_string() {
        let s = String::from("race a car");
        assert!(!is_palindrome_string(s));
    }
}
