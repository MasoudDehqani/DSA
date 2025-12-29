// fn count_duplicates(text: &str) -> u32 {
//     let mut dups = text
//         .chars()
//         .fold(std::collections::HashMap::new(), |mut acc, curr| {
//             let curr = curr.to_ascii_lowercase();
//             acc.entry(curr).and_modify(|v| *v = true).or_insert(false);
//             acc
//         });

//     dups.retain(|_, &mut v| v);
//     dups.len() as u32
// }

// fn count_duplicates(text: &str) -> u32 {
//     text.chars()
//         .fold((std::collections::HashMap::new(), 0), |mut acc, curr| {
//             let curr = curr.to_ascii_lowercase();
//             acc.0
//                 .entry(curr)
//                 .and_modify(|v| {
//                     if *v {
//                         return;
//                     }
//                     acc.1 += 1;
//                     *v = true
//                 })
//                 .or_insert(false);

//             acc
//         })
//         .1
// }

pub fn count_duplicates(text: &str) -> u32 {
    text.chars()
        .fold(std::collections::HashMap::new(), |mut acc, curr| {
            let curr = curr.to_ascii_lowercase();
            acc.entry(curr).and_modify(|v| *v += 1).or_insert(0);
            acc
        })
        .into_iter()
        .filter(|(_, v)| *v > 0)
        .count() as u32
}
