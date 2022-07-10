struct Solution {}

use std::collections::HashSet;
use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut blocks: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for row in 0..(board.len() as usize) {
            for col in 0..(board[row].len() as usize) {
                if board[row][col] == '.' {
                    continue
                }
                match rows.get_mut(&row) {
                    Some(set) => {
                        if set.contains(&board[row][col]) {
                            return false;
                        } else {
                            set.insert(board[row][col]);
                        }
                    },
                    _ => {
                        rows.insert(row, HashSet::from([board[row][col]]));
                    }
                }
                match cols.get_mut(&col) {
                    Some(set) => {
                        if set.contains(&board[row][col]) {
                            return false;
                        } else {
                            set.insert(board[row][col]);
                        }
                    },
                    _ => {
                        cols.insert(col, HashSet::from([board[row][col]]));
                    }
                }
                match blocks.get_mut(&((row / 3) as usize, (col / 3) as usize)) {
                    Some(set) => {
                        if set.contains(&board[row][col]) {
                            return false;
                        } else {
                            set.insert(board[row][col]);
                        }
                    },
                    _ => {
                        blocks.insert(((row / 3) as usize, (col / 3) as usize), HashSet::from([board[row][col]]));
                    }
                }
                println!("{}, {}", row, col);
            }
        }

        true
    }
}

fn main() {
  let board = vec![
      vec![".",".","4",".",".",".","6","3","."],
      vec![".",".",".",".",".",".",".",".","."],
      vec!["5",".",".",".",".",".",".","9","."],
      vec![".",".",".","5","6",".",".",".","."],
      vec!["4",".","3",".",".",".",".",".","1"],
      vec![".",".",".","7",".",".",".",".","."],
      vec![".",".",".","5",".",".",".",".","."],
      vec![".",".",".",".",".",".",".",".","."],
      vec![".",".",".",".",".",".",".",".","."]
  ].iter().map(|v| v.iter().flat_map(|x| x.chars()).collect()).collect();
  let res = Solution::is_valid_sudoku(board);
  // let hoge = vec!["alpha", "beta", "gamma"];
  // let res: Vec<char> = hoge.iter().flat_map(|s| s.chars()).collect();
  println!("{:?}", res);
}

