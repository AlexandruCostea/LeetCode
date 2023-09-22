use std::convert::TryInto;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len()-1).try_into().unwrap();
        let mut mid: i32 = 0;
        let mut pos: usize = 0;
        while left <= right{
            mid = (left+right)/2;
            pos = mid.try_into().unwrap();
            if nums[pos] == target {
                return mid;
            }
            if nums[pos] > target {
                right = mid-1;
            }
            else {
                left = mid+1;
            }
        }
        return left;
    }
}