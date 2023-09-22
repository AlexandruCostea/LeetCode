impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let n: usize = digits.len()-1;
        let mut dig: Vec<i32> = digits.clone();
        let mut remainder: i32 = 0;
        dig[n]+=1;
        for i in (0..=n).rev() {
            dig[i]+=remainder;
            remainder = dig[i]/10;
            dig[i] = dig[i]%10;
        }
        if remainder > 0 {
            dig.insert(0, remainder);
        }
        dig
    }
}