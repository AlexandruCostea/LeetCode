use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap:BinaryHeap<i32> = BinaryHeap::new();
        let mut sum: i64 = 0;
        for elem in nums {
            heap.push(elem);
        }

        for _ in 0..k {
            let max_elem: i32 = heap.pop().unwrap();
            let new_elem: i32 = (max_elem as f64 / 3.0).ceil() as i32;
            heap.push(new_elem);
            sum += max_elem as i64;
        }
        sum
    }
}