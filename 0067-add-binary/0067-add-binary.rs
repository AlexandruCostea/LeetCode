use std::cmp::min;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a: Vec<u8> = a.chars().map(|x| -> u8 {x as u8 - '0' as u8} ).collect();
        a.reverse();
        let mut b: Vec<u8> = b.chars().map(|x| -> u8 {x as u8 - '0' as u8} ).collect();
        b.reverse();
        let mut res: Vec<u8> = Vec::new();

        let mut remainder = 0;
        for i in 0..min(a.len(), b.len()) {
            let x = a[i] + b[i] + remainder;
            res.push(x % 2);
            remainder = x / 2;
        }

        for i in min(a.len(), b.len())..a.len() {
            let x = a[i] + remainder;
            res.push(x % 2);
            remainder = x / 2;
        }

        for i in min(a.len(), b.len())..b.len() {
            let x = b[i] + remainder;
            res.push(x % 2);
            remainder = x / 2;
        }

        if remainder == 1 {
            res.push(1);
        }
        res.reverse();
        return res.iter().map(|&x| -> char {(x as u8 + '0' as u8) as char}).collect();
    }
}