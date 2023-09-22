impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut sol:i32 = 0;
        for character in column_title.chars() {
            sol *= 26;
            sol += (character as i32) - 64;
        }
        return sol;
    }
}