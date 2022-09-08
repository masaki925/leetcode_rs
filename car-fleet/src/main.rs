struct Solution {}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut records: Vec<(i32, i32)> = vec![];
        for i in 0..position.len() {
            records.push((position[i], speed[i]));
        }
        records.sort_by_key(|r| r.0);
        records.reverse();

        let mut stack: Vec<f32> = vec![];
        let mut last_elem = 0.0;

        for (pos, spd) in records {
            let remain_dist = (target - pos) as f32;
            let time_to_target = (remain_dist as f32) / (spd as f32);

            if time_to_target > last_elem {
                stack.push(time_to_target);
                last_elem = time_to_target;
            }
        }
        println!("{:?}", stack);

        stack.len() as i32
    }
}

fn main() {
    let target = 10;
    let position = vec![8,3,7,4,6,5];
    let speed = vec![4,4,4,4,4,4];
    let res = Solution::car_fleet(target, position, speed);
    println!("{}", res);
}

