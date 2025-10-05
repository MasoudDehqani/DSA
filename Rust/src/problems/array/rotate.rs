pub fn rotate_right_k_steps(nums: &mut Vec<i32>, mut k: i32) {
    while k > 0 {
        let last = nums.remove(nums.len() - 1);
        nums.insert(0, last);
        k -= 1;
    }
}
