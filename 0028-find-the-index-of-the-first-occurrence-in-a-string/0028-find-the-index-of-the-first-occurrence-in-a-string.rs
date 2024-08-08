use std::cmp::Ordering;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }
        let haystack: Vec<char> = haystack.chars().collect();
        for j in 0..haystack.len() - needle.len() + 1 {
            let slice: String = haystack[j..j+needle.len()].iter().clone().collect();
            if needle.cmp(&slice) == Ordering::Equal {
                return j as i32;
            }
        }
        return -1;
    }
}