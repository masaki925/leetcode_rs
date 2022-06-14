use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: i32
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::new();

        for n in nums {
            heap.push(Reverse(n));
        }

        while heap.len() > k as usize {
            heap.pop();
        }

        KthLargest {
            heap: heap,
            k: k
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        while self.heap.len() > self.k as usize {
            self.heap.pop();
        }

        self.heap.peek().unwrap().0
    }
}

fn main() {
    let k = 3;
    let nums = vec![4, 5, 8, 2];
    let mut obj = KthLargest::new(k, nums);
    let val = 3;
    let ret_1: i32 = obj.add(val);
    println!("{}", ret_1);
}

