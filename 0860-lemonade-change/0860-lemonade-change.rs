use std::collections::HashMap;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = HashMap::new();
        for bill in bills {
            match bill {
                5 => {
                    let notes = change.entry(5).or_insert(0);
                    *notes += 1;
                },
                x => {
                    let required_change = x - 5;
                    let fives = change.entry(5).or_insert(0).clone();
                    let tens = change.entry(10).or_insert(0).clone();
                    if required_change == 15 {
                        if tens > 0 && fives > 0 {
                            *change.entry(10).or_default() -= 1;
                            *change.entry(5).or_default() -= 1;
                        } else if fives > 2 {
                            *change.entry(5).or_default() -= 3;
                        } else {
                            return false;
                        }
                    } else {
                        if fives > 0 {
                            *change.entry(5).or_default() -= 1;
                        } else {
                            return false;
                        }
                        *change.entry(10).or_default() += 1;
                    }
                }
            }
        }
        return true;
    }
}