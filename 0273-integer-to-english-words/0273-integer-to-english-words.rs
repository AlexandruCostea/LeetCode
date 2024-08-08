use std::collections::HashMap;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        let mut tens = HashMap::new();
        tens.insert(2, "Twenty");
        tens.insert(3, "Thirty");
        tens.insert(4, "Forty");
        tens.insert(5, "Fifty");
        tens.insert(6, "Sixty");
        tens.insert(7, "Seventy");
        tens.insert(8, "Eighty");
        tens.insert(9, "Ninety");

        let mut ones = HashMap::new();
        ones.insert(0, "");
        ones.insert(1, "One");
        ones.insert(2, "Two");
        ones.insert(3, "Three");
        ones.insert(4, "Four");
        ones.insert(5, "Five");
        ones.insert(6, "Six");
        ones.insert(7, "Seven");
        ones.insert(8, "Eight");
        ones.insert(9, "Nine");

        let mut levels = HashMap::new();
        levels.insert(0, "");
        levels.insert(1, "Thousand");
        levels.insert(2, "Million");
        levels.insert(3, "Billion");

        let text = Solution::number_word(num, &ones, &tens, &levels, 0);
        return match text.len() == 0 {
            true => "Zero".to_owned(),
            false => text.trim().to_owned()
        }
    }

    fn number_word(num: i32, ones: &HashMap<i32, &str>, tens: &HashMap<i32, &str>, levels: &HashMap<i32, &str>, level: i32) -> String {
        if num == 0 {
            return "".to_owned();
        }

        let nr = num % 1000;
        let hundreds_digit = nr / 100;
        let tens_digit = (nr / 10) % 10;
        let ones_digit = nr % 10;

        let mut hundred;
        let hundred_text = match hundreds_digit > 0 {
            true => { 
                hundred = ones.get(&hundreds_digit).unwrap().to_owned().to_owned().to_owned() + " Hundred ";
                hundred.as_str()
            },
            false => ""
        };

        let mut nr;
        let tens_text = match tens_digit {
            0 if ones_digit == 0 => "",
            0 => ones.get(&ones_digit).unwrap().to_owned(),
            1 if ones_digit == 0 => "Ten",
            1 if ones_digit == 1 => "Eleven",
            1 if ones_digit == 2 => "Twelve",
            1 if ones_digit == 3 => "Thirteen",
            1 if ones_digit == 4 => "Fourteen",
            1 if ones_digit == 5 => "Fifteen",
            1 if ones_digit == 6 => "Sixteen",
            1 if ones_digit == 7 => "Seventeen",
            1 if ones_digit == 8 => "Eighteen",
            1 if ones_digit == 9 => "Nineteen",
            x => {
                nr = tens.get(&x).unwrap().to_owned().to_owned() + " " + ones.get(&ones_digit).unwrap().to_owned();
                nr.as_str()
            }
        };

        let result =  hundred_text.to_owned() + tens_text;
        let mut result1 = "".to_owned();
        if result.len() > 0 {
            result1 = result.trim().to_owned() + " " + levels.get(&level).unwrap();
        }
        let next_digits = Solution::number_word(num / 1000, ones, tens, levels, level + 1);
        if next_digits.len() == 0 {
            return result1;
        }
        return next_digits.trim().to_owned() + " " + result1.as_str();

    }
}