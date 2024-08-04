use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        const MOD_VAL: i32 = 1_000_000_007;
        let mut sums = BinaryHeap::new();
        let n = n as usize;

        for i in 0..n {
            let mut sum = 0;
            for j in i..n {
                sum += nums[j];
                sum %= MOD_VAL;
                sums.push(Reverse(sum));
            }
        }

        let mut sum = 0;
        let mut i = 1;
        let mut nr = 0;
        while i <= right {
            match sums.pop() {
                Some(x) => nr = x.0,
                None => return sum
            }
            if i >= left {
                sum = (sum + nr) % MOD_VAL;
            }
            i+=1;
        }
        return sum;
    }
}