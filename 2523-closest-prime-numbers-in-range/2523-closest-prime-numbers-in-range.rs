impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut primes: Vec<i32> = vec![0; (right + 1) as usize];
        let mut sol: Vec<i32> = vec![-1, -1];
        let mut minlen: i32 = -1;
        primes[1] = 1;
        let mut start = -1;
        for i in 2..(right+1) / 2 {
            if primes[i as usize] == 0 {
                for j in (i+i..right+1).step_by(i as usize) {
                    primes[j as usize] = 1;
                }
            }
        }
        for i in left..right+1 {
            if primes[i as usize] == 0 {
                if start != -1 && (i - start < minlen || minlen == -1) {
                    minlen = i - start;
                    sol[0] = start;
                    sol[1] = i;
                }
                start = i
            }
        }
        sol
    }
}