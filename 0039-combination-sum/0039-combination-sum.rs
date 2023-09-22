impl Solution {
    fn compute(candidates: &Vec<i32>,cand_pos: usize, target: i32, mut sum: i32, pos: usize, 
        current_solution: &mut Vec<i32>, sol: &mut Vec<Vec<i32>>) {
            for i in cand_pos..candidates.len(){
                current_solution[pos] = candidates[i];
                sum+= candidates[i];
                if sum == target {
                    let new_sol: Vec<i32> = current_solution[0..pos+1].to_vec();
                    sol.push(new_sol);
                }
                else if sum < target {
                    Solution::compute(candidates, i, target, sum, pos+1, current_solution, sol)
                }
                sum -= candidates[i];
            }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        let mut sol: Vec<Vec<i32>> = Vec::new();
        let mut current_sol: Vec<i32> = vec![0; target as usize];
        Solution::compute(&candidates, 0,target, 0, 0, &mut current_sol, &mut sol);
        return sol;
    }
}