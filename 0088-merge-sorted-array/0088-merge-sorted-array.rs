
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut aux: Vec<i32> = Vec::new();
        let mut i: usize = 0;
        let mut j:usize = 0;
        while i < m as usize && j < n as usize{
            match nums1[i] < nums2[j] {
                true => {aux.push(nums1[i]); i+=1;}
                _ => {aux.push(nums2[j]); j+=1;}
            }
        }
        while i < m as usize{
            aux.push(nums1[i]);
            i+=1;
        }
        while j < n as usize {
            aux.push(nums2[j]);
            j+=1;
        }
        nums1.clear();
        nums1.append(&mut aux);
    }
}