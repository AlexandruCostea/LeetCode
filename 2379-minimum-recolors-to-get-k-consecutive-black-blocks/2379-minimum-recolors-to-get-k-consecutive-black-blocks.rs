impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut max_count: i32 = 0;
        let mut count: i32;
        let blocks: Vec<char> = blocks.chars().collect();
        for i in 0..k as usize{
            max_count += (blocks[i] == 'B') as i32;
        }

        count = max_count;
        println!("{}", count);
        for i in k as usize..blocks.len() {
            count -= (blocks[i-k as usize] == 'B') as i32;
            count += (blocks[i] == 'B') as i32;
            println!("{}", count);
            if count > max_count {
                max_count = count;
            }
        }
        k - max_count
    }
}