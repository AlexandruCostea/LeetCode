use std::cmp::max;
use std::collections::BTreeMap;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut mins: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        let mut maxes: BTreeMap<i32, Vec<usize>> = BTreeMap::new();

        let mut max_distance = 0;

        for i in 0..arrays.len() {
            let min_list = mins.entry(arrays[i][0]).or_insert(Vec::new());
            min_list.push(i);

            let max_list = maxes.entry(arrays[i][arrays[i].len() - 1] * -1).or_insert(Vec::new());
            max_list.push(i);
        }

        for maxi in maxes {
            for min in &mins {
                if maxi.1.len() > 1 || min.1.len() > 1 || maxi.1 != *min.1 {
                    max_distance = max(max_distance, maxi.0 * -1 - min.0);
                    break;
                }
            }
        }
        return max_distance;  
    }
}