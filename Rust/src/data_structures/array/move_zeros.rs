pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;

    for j in 0..nums.len() {
        if nums[i] == 0 && nums[j] != 0 {
            nums.swap(i, j);
            i += 1;
        }

        if nums[i] != 0 {
            i += 1;
        }
    }
}
