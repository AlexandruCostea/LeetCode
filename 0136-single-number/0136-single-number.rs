impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut sol: i32 = 0;
        for nr in nums {
            sol^=nr;
        }
        return sol;
    }
}