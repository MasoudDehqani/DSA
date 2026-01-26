// pub fn rotate_right_k_steps(nums: &mut Vec<i32>, mut k: i32) {
//     while k > 0 {
//         let last = nums.remove(nums.len() - 1);
//         nums.insert(0, last);
//         k -= 1;
//     }
// }

// pub fn rotate_right_k_steps(nums: &mut Vec<i32>, k: i32) {
//     let k = (k as usize) % nums.len();
//     // k = if k > nums.len() { k % nums.len() } else { k };
//     // if k == nums.len() {
//     //     return;
//     // }

//     let mut i = 0;
//     let mut j = nums.len() - k;
//     let mut temp = Vec::with_capacity(nums.len() - k);

//     while i < nums.len() {
//         if i < k {
//             temp.push(nums[i]);
//             nums[i] = nums[j];
//         } else {
//             if i < nums.len() - k {
//                 temp.push(nums[i]);
//             }
//         }

//         i += 1;
//         j += 1;
//     }

//     i = 0;
//     for l in k..nums.len() {
//         nums[l] = temp[i];
//         i += 1;
//     }
// }

pub fn left_rotate_by_one_place(arr: &mut [i32]) {
    let mut first = None;
    for i in 0..arr.len() {
        if i == 0 {
            first = Some(arr[i]);
            continue;
        }

        arr[i - 1] = arr[i];
    }

    match first {
        Some(n) => arr[arr.len() - 1] = n,
        None => (),
    }
}

// pub fn left_rotate_by_k_places(arr: &mut [i32], k: u32) {
//     if arr.len() == 0 {
//         return;
//     }

//     let mut k = k % arr.len() as u32;

//     while k > 0 {
//         let mut first = None;
//         for i in 0..arr.len() {
//             if i == 0 {
//                 first = Some(arr[i]);
//                 continue;
//             }

//             arr[i - 1] = arr[i];
//         }

//         match first {
//             Some(n) => arr[arr.len() - 1] = n,
//             None => (),
//         }

//         k -= 1;
//     }
// }

// pub fn left_rotate_by_k_places(arr: &mut [i32], k: u32) {
//     if arr.len() == 0 {
//         return;
//     }

//     let k: usize = k as usize % arr.len();

//     let t = Vec::from(&arr[0..k]);

//     for i in k..arr.len() {
//         arr[i - k] = arr[i];
//     }

//     for i in 0..t.len() {
//         arr[i + k + 1] = t[i];
//     }
// }

// pub fn left_rotate_by_k_places(arr: &mut [i32], k: u32) {
//     if arr.len() == 0 {
//         return;
//     }

//     let k: usize = k as usize % arr.len();

//     let (l, r) = arr.split_at_mut(k);

//     l.reverse();
//     r.reverse();
//     arr.reverse();
// }

pub fn left_rotate_by_k_places(arr: &mut [i32], k: u32) {
    if arr.len() == 0 {
        return;
    }

    let k: usize = k as usize % arr.len();

    for i in 0..k / 2 {
        arr.swap(i, k - 1 - i);
    }

    for i in k..(k + arr.len()) / 2 {
        arr.swap(i, arr.len() - 1 - (i - k));
    }

    for i in 0..arr.len() / 2 {
        arr.swap(i, arr.len() - 1 - i);
    }
}
