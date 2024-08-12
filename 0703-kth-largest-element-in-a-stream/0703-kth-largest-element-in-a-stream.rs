use std::collections::BinaryHeap;

struct KthLargest {
    k: i32,
    nums: BinaryHeap<i32>,
}

impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = KthLargest {
            k,
            nums: BinaryHeap::new(),
        };
        for num in nums {
            if kth_largest.nums.len() < kth_largest.k as usize {
                kth_largest.nums.push(-1 * num);
            } else if num * -1 < *kth_largest.nums.peek().unwrap() {
                kth_largest.nums.push(num * -1);
                kth_largest.nums.pop();
            }

        }
        return kth_largest
    }
    
    fn add(&mut self, val: i32) -> i32 {
        if self.nums.len() < self.k as usize {
            self.nums.push(-1 * val);
        } else if val * -1 < *self.nums.peek().unwrap() {
            self.nums.push(val * -1);
            self.nums.pop();
        }
        return self.nums.peek().unwrap() * -1;
    }
}