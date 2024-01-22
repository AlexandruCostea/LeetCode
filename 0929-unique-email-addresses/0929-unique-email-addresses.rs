use std::collections::hash_set::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for mail in emails {
            let x : Vec<&str> = mail.split("@").collect();
            let receiver = x[0].split("+")
                .next().unwrap()
                .replace(".", "");
            let mail = receiver + "@" + x[1];
            set.insert(mail);
        }
        return set.len() as i32;
    }
}