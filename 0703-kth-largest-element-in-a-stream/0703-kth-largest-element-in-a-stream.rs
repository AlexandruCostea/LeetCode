use std::collections::BTreeMap;

struct KthLargest {
    k: i32,
    nums: BTreeMap<i32, i32>,
}

impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = KthLargest {
            k,
            nums: BTreeMap::new(),
        };
        for num in nums {
            *kth_largest.nums.entry(-1 * num).or_insert(0) += 1;
        }
        return kth_largest
    }
    
    fn add(&mut self, val: i32) -> i32 {
        *self.nums.entry(-1 * val).or_insert(0) += 1;
        let mut i = 0;
        for elem in &self.nums {
            i += elem.1;
            if i >= self.k {
                return elem.0 * -1;
            }
        }
        return 0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */