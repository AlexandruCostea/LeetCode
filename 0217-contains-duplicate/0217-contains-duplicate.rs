use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set:HashSet<i32> = HashSet::new();
        for number in nums {
            if set.contains(&number) {
                return true;
            }
            set.insert(number);
        }
        return false;
    }
}