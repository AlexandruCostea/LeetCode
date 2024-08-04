impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        if target.len() != arr.len() {
            return false;
        }
        
        target.sort();
        arr.sort();
        
        for i in 0..target.len() {
            if target[i] != arr[i] {
                return false;
            }
        }

        return true;
    }
}