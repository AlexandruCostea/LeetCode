impl Solution {
    pub fn min_swaps(mut nums: Vec<i32>) -> i32 {
        let mut nr_ones = 0;
        for nr in &nums {
            nr_ones += *nr as usize;
        }

        nums.append(&mut nums.clone());

        let mut num_ones_slice = 0;

        for i in 0..nr_ones {
            num_ones_slice += nums[i];
        }
        let mut max = num_ones_slice;
        for i in nr_ones..nums.len() {
            num_ones_slice = num_ones_slice - nums[i - nr_ones] + nums[i];
            if num_ones_slice > max {
                max = num_ones_slice;
            }
        }
        return nr_ones as i32 - max;
    }
}