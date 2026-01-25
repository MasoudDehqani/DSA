// pub fn rotate_right_k_steps(nums: &mut Vec<i32>, mut k: i32) {
//     while k > 0 {
//         let last = nums.remove(nums.len() - 1);
//         nums.insert(0, last);
//         k -= 1;
//     }
// }

pub fn rotate_right_k_steps(nums: &mut Vec<i32>, k: i32) {
    let k = (k as usize) % nums.len();
    // k = if k > nums.len() { k % nums.len() } else { k };
    // if k == nums.len() {
    //     return;
    // }

    let mut i = 0;
    let mut j = nums.len() - k;
    let mut temp = Vec::with_capacity(nums.len() - k);

    while i < nums.len() {
        if i < k {
            temp.push(nums[i]);
            nums[i] = nums[j];
        } else {
            if i < nums.len() - k {
                temp.push(nums[i]);
            }
        }

        i += 1;
        j += 1;
    }

    i = 0;
    for l in k..nums.len() {
        nums[l] = temp[i];
        i += 1;
    }
}

pub fn left_rotate_by_one_place(arr: &mut Vec<i32>) {
    match arr.first() {
        Some(&n) => {
            arr.push(n);
            arr.remove(0);
        }
        None => (),
    }
}
