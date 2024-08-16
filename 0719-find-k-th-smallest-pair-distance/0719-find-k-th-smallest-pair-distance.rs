impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut min_distance = 0;
        let mut max_distance = nums[nums.len() - 1] - nums[0];

        while min_distance < max_distance {
            let mid_distance = (min_distance + max_distance) / 2;
            let num_pairs = Solution::num_pairs_within_distance(&nums, mid_distance);
            if num_pairs >= k {
                max_distance = mid_distance;
            } else {
                min_distance = mid_distance + 1;
            }
        }
        return min_distance;
    }

    fn num_pairs_within_distance(nums: &Vec<i32>, distance: i32) -> i32{
        let mut left = 0;
        let mut count = 0;
        for right in 1..nums.len() {
            while(nums[right] - nums[left] > distance) {
                left += 1;
            }
            count += right - left;
        }
        return count as i32;
    }
}