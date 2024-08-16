impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut fives = 0;
        let mut tens = 0;
        for bill in bills {
            match bill {
                5 => {
                    fives += 1;
                },
                x => {
                    let required_change = x - 5;
                    if required_change == 15 {
                        if tens > 0 && fives > 0 {
                            tens -= 1;
                            fives -= 1;
                        } else if fives > 2 {
                            fives -= 3;
                        } else {
                            return false;
                        }
                    } else {
                        if fives > 0 {
                            fives -= 1;
                        } else {
                            return false;
                        }
                        tens += 1;
                    }
                }
            }
        }
        return true;
    }
}