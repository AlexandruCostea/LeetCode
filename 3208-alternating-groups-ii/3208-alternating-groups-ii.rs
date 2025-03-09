impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut groups: Vec<usize> = Vec::new();
        let mut start: usize = 0;
        let mut sol: i32 = 0;
        let mut begin_group: bool = false;
        let mut end_group: bool = false;
        for i in 1..colors.len() {
            if colors[i] == colors[i-1] {
                if i - 1 - start > 0 {
                    groups.push(i - 1 - start + 1);
                    if start == 0 {
                        begin_group = true;
                    }
                }
                start = i;
            }
        }
        if colors[colors.len() - 1] != colors[colors.len() - 2] {
            groups.push(colors.len() - start);
            end_group = true;
        }

        if begin_group && end_group && groups.len() > 1 && colors[0] != colors[colors.len() - 1] {
            groups[0] += groups.pop().unwrap();

        }

        else if groups.len() == 1 && groups[0] == colors.len() && colors[0] != colors[colors.len() - 1] {
                groups[0] = groups[0] + k as usize - 1;
        }

        else if (begin_group || end_group) && colors[0] != colors[colors.len() - 1] {
            let n: usize = groups.len();
            if begin_group {
                groups[0] += 1;
            }
            else {
                groups[n - 1] += 1;
            }
        }

        for elem in groups {
            if elem as i32 >= k  {
                sol += elem as i32 - k + 1;
            }
        }

        sol
    }
}