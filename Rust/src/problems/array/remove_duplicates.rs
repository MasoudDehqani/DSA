pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut i = 0;
    while i < nums.len() {
        if prev == nums[i] {
            nums[i] = nums[i];
        } else {
            prev = nums[i];
            i += 1;
        }
    }

    nums.len().try_into().unwrap()
}
