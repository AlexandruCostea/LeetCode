use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        let n:i32 = nums.len() as i32;
        for number in nums {
            let count: &mut i32 = counts.entry(number).or_insert(0);
            *count+=1;
            if *count > n/2 {
                return number;
            }
        }
        return 0;
    }
}