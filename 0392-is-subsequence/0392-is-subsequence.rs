impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let str1: Vec<char> = s.chars().collect();
        let str2: Vec<char> = t.chars().collect();
        let mut i = 0;
        let mut j = 0;
        while i < str1.len() && j < str2.len() {
            while j < str2.len() && str2[j] != str1[i]{
                j+=1;
            }
            if i < str1.len() && j < str2.len() && str2[j] == str1[i] {
                i+=1;
                j+=1;
            }
        }
        return i == str1.len();
    }
}