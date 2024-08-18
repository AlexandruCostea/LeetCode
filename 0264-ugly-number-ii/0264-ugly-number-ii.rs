use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_numbers: BinaryHeap<i32> = BinaryHeap::new();
        let mut visited = HashSet::new();
        ugly_numbers.push(-1);
        visited.insert(-1);
        let mut i = 0;
        let mut number = 0;
        while i < n {
            number = ugly_numbers.pop().unwrap() * -1;
            i += 1;
            if  number as i64 * 2 < i32::MAX as i64 && !visited.contains(&(number * -2)) {
                ugly_numbers.push(number * -2);
                visited.insert(number * -2);
            }
            if number as i64 * 3 < i32::MAX as i64 && !visited.contains(&(number * -3)) {
                ugly_numbers.push(number * -3);
                visited.insert(number * -3);
            }
            if number as i64 * 5 < i32::MAX as i64 && !visited.contains(&(number * -5)) {
                ugly_numbers.push(number * -5);
                visited.insert(number * -5);
            }
        }
        return number;
    }
}