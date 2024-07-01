pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 1;
        for i in 1..nums.len() {
            if count == 1 || nums[i] != nums[count - 2] {
                nums[count] = nums[i];
                count += 1;
            }
        }

        println!("{:?}", nums);
        return count as i32;
    }
}
