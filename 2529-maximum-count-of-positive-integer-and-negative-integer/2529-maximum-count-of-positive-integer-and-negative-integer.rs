impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut pos: i32 = 0;
        let mut neg: i32 = 0;
        for elem in nums {
            if elem > 0 {
                pos += 1;
            }
            else if elem < 0 {
                neg += 1;
            }
        }

        match pos > neg {
            true => pos,
            _ => neg
        }
    }
}