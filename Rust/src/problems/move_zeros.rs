pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(arr.len());
    let mut zeros = 0;
    for &n in arr {
        if n != 0 {
            res.push(n)
        } else {
            zeros += 1;
        }
    }

    let mut zeros = vec![0; zeros];
    res.append(&mut zeros);
    res
}

// a more concise solution
// pub fn move_zeros(arr: &[u8]) -> Vec<u8> {
//     let mut res = arr.iter().copied().filter(|&n| n != 0).collect::<Vec<u8>>();
//     res.resize(arr.len(), 0);

//     res
// }
