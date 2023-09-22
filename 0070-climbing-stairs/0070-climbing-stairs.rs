impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            3 => 3,
            m => {
                let mut x = 3;
                let mut y = 5;
                let mut pos = 4;
                while pos < m {
                    pos+=1;
                    let aux = x + y;
                    x = y;
                    y = aux;
                }
                y
            }
        }
    }
}