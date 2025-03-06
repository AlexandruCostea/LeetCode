use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0, 0];
        let n: i32 = grid.len() as i32;
        let mut missing: HashSet<i32> = HashSet::new();
        let mut found: HashMap<i32,i32> = HashMap::new();
        for i in 1..n*n + 1 {
            missing.insert(i);
        }

        for row in grid {
            for elem in row {
                missing.remove(&elem);
                *(found.entry(elem).or_insert(0))+=1;
            }
        }

        res[1] = *missing.iter().next().unwrap();
        
        for elem in found {
            if elem.1 > 1 {
                res[0] = elem.0;
            }
        }

        res
    }
}