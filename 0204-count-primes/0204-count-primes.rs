impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut primes:Vec<bool> = vec![true; n as usize];
        let mut count = 0;
        for i in 2..=n-1 {
            if primes[i as usize] == true {
                count+=1;
                for j in (i*2..=n-1).step_by(i as usize) {
                    primes[j as usize] = false;
                }
            }
        }
        return count;
    }
}