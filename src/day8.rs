use anyhow::Result;
use std::fs;
use std::cmp::max;

fn main() -> Result<()> {
    let map: Vec<Vec<i32>> = parse_map(fs::read_to_string("day8.input")?.as_str());
    println!("{}", part1(&map));
    println!("{}", part2(&map));
    Ok(())
}

fn parse_map(input: &str) -> Vec<Vec<i32>> {
    input.split("\n")
      .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).map(|v| v as i32).collect::<Vec<i32>>())
      .collect()
}

fn part1(map: &Vec<Vec<i32>>) -> u32 {
    let n: usize = map.len();
    let m: usize = map[0].len();
    let mut left: Vec<Vec<i32>> = vec![vec![-1; m]; n];
    for i in 0..n {
        for j in 1..m {
            left[i][j] = max(left[i][j - 1], map[i][j - 1])
        }
    }
    let mut right: Vec<Vec<i32>> = vec![vec![-1; m]; n];
    for i in 0..n {
        for j in (1..m).rev() {
            right[i][j - 1] = max(right[i][j], map[i][j])
        }
    }
    let mut top: Vec<Vec<i32>> = vec![vec![-1; m]; n];
    for i in 1..n {
        for j in 0..m {
            top[i][j] = max(top[i - 1][j], map[i - 1][j])
        }
    }
    let mut bottom: Vec<Vec<i32>> = vec![vec![-1; m]; n];
    for i in (1..n).rev() {
        for j in 0..m {
            bottom[i - 1][j] = max(bottom[i][j], map[i][j])
        }
    }
    let mut tree: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] > left[i][j] 
                || map[i][j] > right[i][j]
                || map[i][j] > top[i][j]
                || map[i][j] > bottom[i][j] {
                    tree = tree + 1;
                }
        }
    }
    tree
}

fn part2(map: &Vec<Vec<i32>>) -> u32 {
    let n: usize = map.len();
    let m: usize = map[0].len();
    let mut max_score: u32 = 0;
    for i in 1..(n-1) {
        for j in 1..(m-1) {
            let mut left = 0;
            for k in (0..j).rev() {
                left = left + 1;
                if map[i][k] >= map[i][j] {
                    break
                }
            }
            let mut right = 0;
            for k in (j+1)..m {
                right = right + 1;
                if map[i][k] >= map[i][j] {
                    break
                }
            }
            let mut top = 0;
            for k in (0..i).rev() {
                top = top + 1;
                if map[k][j] >= map[i][j] {
                    break
                }
            }
            let mut bottom = 0;
            for k in (i+1)..n {
                bottom = bottom + 1;
                if map[k][j] >= map[i][j] {
                    break
                }
            }
            max_score = max(max_score, left * right * top * bottom);
        }
    }
    max_score
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373\n\
                         25512\n\
                         65332\n\
                         33549\n\
                         35390";

    #[test]
    fn test1() {
        let result: u32 = part1(&parse_map(INPUT));
        assert_eq!(result, 21);
    }

    #[test]
    fn test2() {
        let result: u32 = part2(&parse_map(INPUT));
        assert_eq!(result, 8);
    }
}