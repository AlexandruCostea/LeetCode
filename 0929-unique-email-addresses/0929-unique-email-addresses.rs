use std::collections::hash_set::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for mail in emails {
            let x : Vec<&str> = mail.split("@").collect();
            let receiver:Vec<&str> = x[0].split("+").collect();
            let receiver:String = String::from_utf8(
                receiver[0].as_bytes().iter()
                    .map(|&x| {x})
                    .filter(|&x| {x != b'.'})
                    .collect()).unwrap();
            let mail = receiver + "@" + x[1];
            set.insert(mail);
        }
        return set.len() as i32;
    }
}