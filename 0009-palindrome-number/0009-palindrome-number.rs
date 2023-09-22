use std::cmp::Ordering;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut reverse: i64 = 0;
        let mut aux: i64 = x.into();
        let x: i64 = x.into();
        if(aux < 0) {
            return false;
        }
        while(aux != 0) {
            reverse *= 10;
            reverse +=aux%10;
            aux/=10;
        }
        return match reverse.cmp(&x) {
            Ordering::Equal => true,
            Ordering::Less => false,
            Ordering::Greater=> false,
        }
    }
}