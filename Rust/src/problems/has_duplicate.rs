// pub fn has_duplicate(arr: &[i32]) -> bool {
//     for (i, n) in arr.iter().enumerate() {
//         for (j, n2) in arr.iter().enumerate() {
//             if i != j && n == n2 {
//                 return true;
//             }
//         }
//     }

//     false
// }

pub fn has_duplicate(arr: &[i32]) -> bool {
    let mut temp = vec![];
    for n in arr {
        if temp.contains(n) {
            return true;
        }
        temp.push(*n);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate() {
        assert_eq!(has_duplicate(&[1, 2, 3, 4, 5]), false);
        assert_eq!(has_duplicate(&[1, 2, 3, 4, 5, 5]), true);
        assert_eq!(has_duplicate(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), false);
        assert_eq!(has_duplicate(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10]), true);
    }
}
