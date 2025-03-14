use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut frequency: HashMap<i32, i32> = HashMap::new();
        let mut values: BinaryHeap<i32> = BinaryHeap::new();
        let mut leftover: i64 = 0;
        let mut elem: i32;

        for elem in candies {
            *frequency.entry(elem).or_default() += 1;
            values.push(elem);
        }

        while !values.is_empty() {
            elem = values.pop().unwrap();
            leftover += *frequency.get(&elem).unwrap() as i64;

            if leftover >= k {
                return elem;
            }
        }
        0
    }
}