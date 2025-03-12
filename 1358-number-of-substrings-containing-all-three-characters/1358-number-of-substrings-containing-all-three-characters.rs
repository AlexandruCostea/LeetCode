use std::collections::HashMap;


impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut sol: usize = 0;
        let word: Vec<char> = s.chars().collect();
        let mut frequency: HashMap<char, i32> = HashMap::new();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut prevj: usize = 0;
        let mut previ: usize = 0;
        let mut found: bool;

        frequency.insert('a', 0);
        frequency.insert('b', 0);
        frequency.insert('c', 0);

        while i < word.len() {
            found = true;
            while (*frequency.get(&'a').unwrap() == 0 || *frequency.get(&'b').unwrap() == 0 || *frequency.get(&'c').unwrap() == 0) && j < word.len() {
                *frequency.entry(word[j]).or_default() += 1;
                j += 1;
            }

            if *frequency.get(&'a').unwrap() == 0 || *frequency.get(&'b').unwrap() == 0 || *frequency.get(&'c').unwrap() == 0 {
                found = false;
            }

            j -= 1;
            
            println!("{i} {j} {:?}", frequency);

            while *frequency.get(&'a').unwrap() > 0 && *frequency.get(&'b').unwrap() > 0 && *frequency.get(&'c').unwrap() > 0 {
                *frequency.entry(word[i]).or_default() -= 1;
                i += 1;

                if *frequency.get(&'a').unwrap() > 0 && *frequency.get(&'b').unwrap() > 0 && *frequency.get(&'c').unwrap() > 0 {
                    sol += word.len() - j;
                }
            }
            println!("{i} {j} {:?}\n", frequency);
            j+= 1;

            if i == previ && j == prevj {
                return sol as i32;
            }

            if found {
                sol += word.len() - j + 1;
            }
            previ = i;
            prevj = j;
        }
        sol as i32
    }
}