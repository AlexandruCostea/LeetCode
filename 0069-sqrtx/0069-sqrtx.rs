use std::convert::TryInto;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut sqr:i64 = 0;
        let y: i64 = x.into();
        while sqr * sqr <= y {
            sqr+=1;
        }
        (sqr-1).try_into().unwrap()
    }
}