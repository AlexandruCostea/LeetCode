use std::collections::HashSet;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut distinct: HashSet<&String> = HashSet::new();
        let mut repeated: HashSet<&String> = HashSet::new();

        for elem in &arr {
            if distinct.contains(&elem) {
                distinct.remove(&elem);
                repeated.insert(elem);
            } else if !repeated.contains(&elem) {
                distinct.insert(elem);
            }
        }

        let mut i = 0;
        for elem in &arr {
            if distinct.contains(&elem) {
                i+=1;
                if i == k {
                    return elem.to_owned()
                }
            }
        }
        return "".to_owned();
    }
}