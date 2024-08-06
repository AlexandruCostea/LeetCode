impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut count = 0;
        for passenger in details {
            let age = passenger[11..13].parse::<i32>().unwrap();
            if age > 60 {
                count+=1;
            }
        }
        return count;
    }
}