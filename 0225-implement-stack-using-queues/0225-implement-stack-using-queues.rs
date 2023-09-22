use std::collections::VecDeque;
use std::mem::replace;
struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
    pos: bool,
    last: Option<i32>,
    size: i32
}

impl MyStack {

    fn new() -> Self {
        MyStack {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
            pos: true,
            last: None,
            size: 0
        }
    }
    
    fn push(&mut self, x: i32) {
        self.size+=1;
        match self.pos {
            true => {
                self.q1.push_back(x);
                self.last = Some(x);
            }
            false => {
                self.q2.push_back(x);
                self.last = Some(x);
            }
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.size-=1;
        match self.pos {
            true => {
                self.pos = false;
                while self.q1.len() > 1 {
                    let x: Option<&i32> = replace(&mut self.q1.front(), Some(&0));
                    match x {
                        Some(y) => {
                            self.q2.push_back(*y);
                            self.last = Some(*y);
                        },
                        None => break
                    }
                    self.q1.pop_front();
                }
                let x:i32 = *(replace(&mut self.q1.front(), Some(&0)).unwrap());
                self.q1.pop_front();
                return x;
            }
            false => {
                self.pos = true;
                while self.q2.len() > 1 {
                    let x: Option<&i32> = replace(&mut self.q2.front(), Some(&0));
                    match x {
                        Some(y) => {
                            self.q1.push_back(*y);
                            self.last = Some(*y);
                        },
                        None => break
                    }
                    self.q2.pop_front();
                }
                let x: i32= *(replace(&mut self.q2.front(), Some(&0)).unwrap());
                self.q2.pop_front();
                return x;
            }
        }
    }
    
    fn top(&self) -> i32 {
        return match self.last {
            Some(x) => x,
            None => -1,
        }
    }
    
    fn empty(&self) -> bool {
        return self.size == 0;
    }
}