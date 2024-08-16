impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        candidates.sort_unstable();
        let mut candidate: Vec<i32> = Vec::new();
        Solution::find_solutions(&candidates, 0, &mut candidate, target, &mut result);
        return result;
    }

    fn find_solutions(nums: &Vec<i32>, index: usize, sol: &mut Vec<i32>, target: i32, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(sol.clone());
            return;
        }

        if target < 0 {
            return;
        }
        for i in index..nums.len() {
            if i > index && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] > target {
                break;
            }

            sol.push(nums[i]);
            let new_target = target - nums[i];
            Solution::find_solutions(nums, i+1, sol, new_target, result);
            sol.remove(sol.len() - 1);
        }
    }
}