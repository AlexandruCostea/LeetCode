// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut start: Box<ListNode>;
        match head {
            Some(node) => start = node,
            None => return None
        }

        let mut head = &mut start;
        let mut mid = 1;

        while let Some(_) = head.next.as_mut() {
            mid+=1;
            head = head.next.as_mut().unwrap();
        }

        mid /= 2;
        let mut pos = 1;
        while pos <= mid {
            pos+=1;
            start = start.next.unwrap();
        }

        Some(start)
    }
}