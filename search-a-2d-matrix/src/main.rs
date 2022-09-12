struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut rl = 0;
        let mut rr = rows - 1;

        let mut row = -1;

        while rl <= rr {
            let rmid = (rl + rr) / 2;

            if target == matrix[rmid][0] {
                row = rmid as i32;
                break;
            } else if target < matrix[rmid][0] {
                rr = rmid - 1;
                if rr >= 0 && target >= matrix[rr][0] {
                    row = rr as i32;
                    break;
                }
            } else if target > matrix[rmid][0] {
                if target <= matrix[rmid][cols-1] {
                    row = rmid as i32;
                    break;
                }
                rl = rmid + 1;
            }
        }

        if row == -1 { return false }

        let mut cl = 0;
        let mut cr = cols - 1;

        while cl <= cr {
            let cmid = (cl + cr) / 2;

            if target == matrix[row as usize][cmid] {
                return true;
            } else if target < matrix[row as usize][cmid] {
                cr = cmid - 1;
            } else if target > matrix[row as usize][cmid] {
                cl = cmid + 1;
            }
        }

        false
    }
}

fn main() {
    let matrix = vec![
        vec![1,3,5,7],
        vec![10,11,16,20],
        vec![23,30,34,60],
    ];

    let target = 13;
    let res = Solution::search_matrix(matrix, target);
    println!("{}", res);
}

