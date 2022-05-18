use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut res = Vec::new();

        let mut short = nums1;
        let mut long = nums2;
        if short.len() > long.len() {
            let tmp = short;
            short = long;
            long = tmp;
        }

        for n in short {
            let counter = map.entry(n).or_insert(0);
            *counter += 1;
        }

        for n in long {
            if map.contains_key(&n) && map[&n] > 0 {
                res.push(n);
                map.entry(n).and_modify(|e| *e -= 1);
            }
        }

        return res;
    }
}

fn main() {
    let nums1 = vec![1,2,2,1,3,4];
    let nums2 = vec![2,2];
    let result = Solution::intersection(nums1, nums2);
    println!("{:?}", result);
}

