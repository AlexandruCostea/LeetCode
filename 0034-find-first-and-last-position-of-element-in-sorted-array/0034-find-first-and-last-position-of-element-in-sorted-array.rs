impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1,-1];
        }
        let mut i: i32 = 0;
        let mut j: i32 = (nums.len()-1) as i32;
        let mut poz: i32 = -1;
        while i <= j {
            let mid: i32 = (i+j)/2;
            if nums[mid as usize] == target {
                poz = mid;
                break;
            }
            else {
                if nums[mid as usize] < target {
                    i = mid+1;
                }
                else {
                    j = mid-1;
                }
            }
        }
        if poz == -1 {
            return vec![-1, -1];
        }
        i = poz;
        j = poz;
        while i >= 0 && nums[i as usize] == nums[poz as usize] {
            i-=1;
        }
        while j < nums.len() as i32 && nums[j as usize] == nums[poz as usize] {
            j+=1;
        }
        return vec![i+1, j-1];
    }
}