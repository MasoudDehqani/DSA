pub fn check(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut drops = 0;

    for i in 1..len {
        if nums[i] < nums[i - 1] {
            drops += 1;
            if drops > 1 {
                return false;
            }
        }
    }

    if drops == 1 && nums[0] < nums[len - 1] {
        return false;
    }

    true
}
