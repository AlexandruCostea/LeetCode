use std::convert::TryInto;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let x: std::str::Split<'_, &str> = s.trim().split(" ");
        let mut last_word: &str = "";
        for word in x {
            last_word = word;
        }
        return last_word.len().try_into().unwrap();
    }
}