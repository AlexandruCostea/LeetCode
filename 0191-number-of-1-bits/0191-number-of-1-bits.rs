impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut bit: u32 = 1;
        let mut count: i32 = 0;
        for _ in 0..32 {
            match n & bit {
                0 => (),
                _ => count = count + 1,
            }
            bit = bit << 1;
        }
        return count;
    }
}