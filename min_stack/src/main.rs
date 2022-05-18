struct MinStack {
    stack: Vec<i32>,
    mins: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack { stack: vec![], mins: vec![] }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() || val <= self.get_min() {
            self.mins.push(val);
        }
        self.stack.push(val);
    }

    fn pop(&mut self) {
        if self.stack.pop() == Some(self.get_min()) {
            self.mins.pop();
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 */
fn main() {
 let mut obj = MinStack::new();
 obj.push(3);
 obj.push(2);
 obj.push(1);
 obj.pop();
 let ret_3: i32 = obj.top();
 let ret_4: i32 = obj.get_min();
 println!("{}", ret_3);
 println!("{}", ret_4);
}

