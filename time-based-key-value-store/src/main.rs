use std::collections::HashMap;

struct TimeMap {
    data: HashMap<String, Vec<(String, i32)>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap{
            data: HashMap::new()
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let d = self.data.entry(key).or_insert(vec![]);
        d.push((value, timestamp));
    }

    fn get(&mut self, key: String, timestamp: i32) -> String {
        let values = self.data.entry(key).or_insert(vec![]);
        let (mut l, mut r) = (0, values.len() - 1);
        let mut res = "";

        while l <= r && r < values.len() {
            let m = (l + r) / 2;

            if values[m].1 <= timestamp {
                res = &values[m].0;
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        res.to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

struct Solution {}

impl Solution {
    pub fn a(instructions: Vec<&str>, args: Vec<Vec<&str>>) -> Vec<String> {
        let mut tm = TimeMap::new();
        let mut res: Vec<String> = vec![];

        for i in 0..instructions.len() {
            if i == 0 {
                res.push("".to_string());
                continue;
            }

            if i == instructions.len() - 1 {
                println!("{:?}", tm.data);
            }


            if instructions[i] == "set" {
                tm.set(args[i][0].to_string(), args[i][1].to_string(), args[i][2].parse().unwrap());
                res.push("".to_string());
            } else if instructions[i] == "get" {
                let s = tm.get(args[i][0].to_string(), args[i][1].parse().unwrap());
                res.push(s);
            }
        }

        res
    }
}

fn main() {
    
    
    // [null,null,null,null,null,"ztpearaw","gszaw","gszaw","ztpearaw","hwliiq","gszaw",null,"kcvcjzzwx","gszaw","kcvcjzzwx",null,null,null,null,"hwliiq",""];
    // [null,null,null,null,null,"ztpearaw","gszaw","gszaw","ztpearaw","hwliiq","gszaw",null,"kcvcjzzwx","gszaw","kcvcjzzwx",null,null,null,null,"hwliiq","zondoubtib"];

    let instructions = vec!["TimeMap","set","set","set","set","get","get","get","get","get","get","set","get","get","get","set","set","set","set","get","get"];
    let args = vec![
        vec!["_", "_", "_"],
        vec!["ctondw","ztpearaw","1"],
        vec!["vrobykydll","hwliiq","2"],
        vec!["gszaw","ztpearaw","3"],
        vec!["ctondw","gszaw","4"],
        vec!["gszaw","5"],
        vec!["ctondw","6"],
        vec!["ctondw","7"],
        vec!["gszaw","8"],
        vec!["vrobykydll","9"],
        vec!["ctondw","10"],
        vec!["vrobykydll","kcvcjzzwx","11"],
        vec!["vrobykydll","12"],
        vec!["ctondw","13"],
        vec!["vrobykydll","14"],
        vec!["ztpearaw","zondoubtib","15"],
        vec!["kcvcjzzwx","hwliiq","16"],
        vec!["wtgbfvg","vrobykydll","17"],
        vec!["hwliiq","gzsiivks","18"],
        vec!["kcvcjzzwx","19"],
        vec!["ztpearaw","20"]];

    let res = Solution::a(instructions, args);
    println!("{:?}", res);
}

