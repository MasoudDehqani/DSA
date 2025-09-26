use super::reverse_number;

pub fn is_palindrome_number(n: i32) -> bool {
    if n < 0 {
        return false;
    }

    reverse_number(n) == n
}
