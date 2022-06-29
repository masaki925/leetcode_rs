struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

        for s in strs {
            let mut key = [0; 26].to_vec();
            for c in s.chars().into_iter() {
                let offset = c as u16 - 'a' as u16;
                key[offset as usize] += 1;
            }
            hm.entry(key).or_insert(vec![]).push(s);
        }
        hm.into_values().collect()
    }
}

fn main() {
  //let strs = vec!["eat","tea","tan","ate","nat","bat"];
  let strs = vec![""];
  let strss: Vec<String> = strs.iter().map(|w| String::from(*w)).collect();
  let res = Solution::group_anagrams(strss);
  println!("{:?}", res);
}

