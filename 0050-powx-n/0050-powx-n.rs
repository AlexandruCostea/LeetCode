impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n.cmp(&0) {
            std::cmp::Ordering::Equal => 1.0,
            std::cmp::Ordering::Less => 1.0/(x * Solution::my_pow(x, -1*n-1)),
            std::cmp::Ordering::Greater => {
                if n % 2 == 1 {
                    return x * Solution::my_pow(x, n-1);
                }
                else {
                    let pw: f64 = Solution::my_pow(x, n/2);
                    return pw * pw;
                }
            }
        }
    }
}