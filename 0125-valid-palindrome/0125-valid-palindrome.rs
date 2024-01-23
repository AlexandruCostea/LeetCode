impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let text:String = s.to_lowercase().split(|c: char| {!c.is_alphanumeric()}).collect();
        let mut reverse = text.clone().as_bytes().to_owned();
        reverse.reverse();
        let reverse = String::from_utf8(reverse.to_vec()).unwrap();

        return text.eq(&reverse);
    }
}