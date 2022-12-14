use anyhow::Result;
use std::fs;
use serde_json::Value;
use serde_json::Value::Number;
use serde_json::Value::Array;
use serde_json::json;
use std::cmp::Ordering;


fn main() -> Result<()> {
    let pairs: Vec<Value> = parse_input(fs::read_to_string("day13.input")?.as_str());
    println!("{}", part1(&pairs));
    println!("{}", part2(&pairs));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Value> {
    input.split("\n")
      .filter(|s| !s.is_empty())
      .flat_map(|s| serde_json::from_str(s))
      .collect()
}

fn part1(pairs: &Vec<Value>) -> usize {
    pairs.chunks(2)
      .enumerate()
      .map(|(index, pair)| (index, order(&pair[0], &pair[1])))
      .filter(|(_, order)| *order == Ordering::Less)
      .map(|(index, _)| index + 1)
      .sum()
}

fn part2(pairs: &Vec<Value>) -> usize {
    let mut elements = pairs.clone();
    let key1: Value = Array(vec![Array(vec![json!(2)])]);
    let key2: Value = Array(vec![Array(vec![json!(6)])]);
    elements.push(key1.clone());
    elements.push(key2.clone());
    elements.sort_by(|a, b| order(a, b));
    elements.iter()
      .enumerate()
      .filter(|(_, element)| **element == key1 || **element == key2)
      .map(|(index, _)| index + 1)
      .product()
}

fn order(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Number(x), Number(y)) => x.as_i64().unwrap().cmp(&y.as_i64().unwrap()),
        (Array(vec_x), Array(vec_y)) => order_vec(vec_x, vec_y),
        (x, Array(vec_y)) => order_vec(&vec![x.clone()], vec_y),
        (Array(vec_x), y) => order_vec(vec_x, &vec![y.clone()]),
        _ => panic!(),
    }
}

fn order_vec(a: &Vec<Value>, b: &Vec<Value>) -> Ordering {
    let mut it_xs = a.iter();
    let mut it_ys = b.iter();
    let mut result = Ordering::Equal;
    loop {
        result = match (it_xs.next(), it_ys.next()) {
            (Some(x), Some(y)) => order(x, y),
            (Some(_), None) => Ordering::Greater,  
            (None, Some(_)) => Ordering::Less, 
            (None, None) => break,
        };
        if result != Ordering::Equal {
            break
        }
    }
    result
}

// cargo test --bin day10 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, 13);
    }

    #[test]
    fn test2() {
        let result = part2(&parse_input(&INPUT));
        assert_eq!(result, 140);
    }

    const INPUT: &str ="[1,1,3,1,1]\n\
                        [1,1,5,1,1]\n\
                        \n\
                        [[1],[2,3,4]]\n\
                        [[1],4]\n\
                        \n\
                        [9]\n\
                        [[8,7,6]]\n\
                        \n\
                        [[4,4],4,4]\n\
                        [[4,4],4,4,4]\n\
                        \n\
                        [7,7,7,7]\n\
                        [7,7,7]\n\
                        \n\
                        []\n\
                        [3]\n\
                        \n\
                        [[[]]]\n\
                        [[]]\n\
                        \n\
                        [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
                        [1,[2,[3,[4,[5,6,0]]]],8,9]";
}