impl Solution {
        pub fn to_lower_case(s: String) -> String {
        let rez: Vec<u8> = s.as_bytes().iter()
            .map(|&x| {if x >= b'A' && x <= b'Z' {x + b'a' - b'A'} else {x}})
            .collect();
        String::from_utf8(rez).unwrap()
    }
}