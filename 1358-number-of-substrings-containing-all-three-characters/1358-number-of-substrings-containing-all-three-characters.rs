use std::collections::HashMap;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut sol: usize = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut letters: HashMap<char, i32> = HashMap::new();
        let mut i: usize = 0;
        let mut j: usize = 0;
        letters.insert('a', 0);
        letters.insert('b', 0);
        letters.insert('c', 0);
        while i < chars.len() {
            while (*letters.get(&'a').unwrap() == 0 || *letters.get(&'b').unwrap() == 0 || *letters.get(&'c').unwrap() == 0) && j< chars.len() {
                *letters.entry(chars[j]).or_default() += 1;
                j += 1;
            }
            if (*letters.get(&'a').unwrap() == 0 || *letters.get(&'b').unwrap() == 0 || *letters.get(&'c').unwrap() == 0) && j == chars.len(){
                break;
            }
            sol += chars.len() - j + 1;
            *letters.entry(chars[i]).or_default() -= 1;
            i +=1;
        }

        (sol) as i32
    }
}