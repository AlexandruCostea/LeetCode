use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
           let mut letter_frequency = HashMap::new();
           let letters: HashSet<char> = word.chars().collect();
           let mut letters: Vec<&char> = letters.iter().collect();
           for letter in word.chars() {
               let frequency = letter_frequency.entry(letter).or_insert(0);
               *frequency += 1;
           }
           letters.sort_by(|x, y| letter_frequency.get(*y).unwrap().cmp(&letter_frequency.get(*x).unwrap()));
           let mut i = 0;
           let mut answer = 0;
           for letter in letters {
              answer += letter_frequency.get(letter).unwrap() * (i/8+1);
              i+=1;
           }
           return answer;
    }
}