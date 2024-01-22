use std::collections::{hash_map, HashMap};

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut map:HashMap<i32, i32> = hash_map::HashMap::new();
        for num in nums {
            let freq = map.entry(num).or_insert(0);
            *freq += 1;
        }
        let res: Vec<i32> = map.iter().filter(|x| {*(*x).1 > 1})
            .map(|x| x.0.to_owned()).collect();
        let mut missing = 0;
        for num in (1..10000) {
            if !map.contains_key(&num) {
                missing = num;
                break;
            }
        }
        return vec![res.first().unwrap_or_else(|| {&-1}).to_owned(), missing];
    }
}