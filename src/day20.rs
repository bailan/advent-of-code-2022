use anyhow::Result;
use std::fs;


fn main() -> Result<()> {
    let numbers: Vec<(usize, i64)> = parse_input(fs::read_to_string("day20.input")?.as_str());
    println!("{}", part1(&numbers));
    println!("{}", part2(&numbers));
    Ok(())
}

fn parse_input(input: &str) -> Vec<(usize, i64)> {
    input.split("\n").map(|number| number.parse::<i64>().unwrap()).collect::<Vec<i64>>()
        .iter().enumerate().map(|(index, number)| (index, *number)).collect()
}

fn part1(numbers: &Vec<(usize, i64)>) -> i64 {
    let n = numbers.len();
    let mut mutable_numbers = numbers.clone();
    for initial_index in 0..n {
        let current_index = mutable_numbers.iter().enumerate().find(|(_, (index, _))| *index == initial_index).unwrap().0;
        let step_forward = mutable_numbers[current_index].1.rem_euclid((n - 1) as i64) as usize;
        let next_index = (current_index + step_forward) % n;
        if current_index <= next_index {
            move_forward(&mut mutable_numbers, current_index, next_index);
        } else {
            move_backward(&mut mutable_numbers, current_index, next_index);
        }
    }

    let zero_index = mutable_numbers.iter().enumerate().find(|(_, (_, value))| *value == 0).unwrap().0;
    mutable_numbers[(zero_index + 1000) % n].1 + mutable_numbers[(zero_index + 2000) % n].1 + mutable_numbers[(zero_index + 3000) % n].1
}

fn move_forward(array: &mut Vec<(usize, i64)>, index: usize, next_index: usize) -> () {
    let first = array[index].clone();
    for i in index..next_index {
        array[i] = array[i + 1];
    }
    array[next_index] = first;
}

fn move_backward(array: &mut Vec<(usize, i64)>, index: usize, previous_index: usize) -> () {
    let last = array[index].clone();
    for i in ((previous_index + 1)..index).rev() {
        array[i + 1] = array[i];
    }
    array[previous_index + 1] = last;
}

fn part2(numbers: &Vec<(usize, i64)>) -> i64 {
    let n = numbers.len();
    let mut mutable_numbers: Vec<(usize, i64)> = numbers.iter().map(|(index, number)| (index.clone(), number * 811589153)).collect();
    for _ in 0..10 {
        for initial_index in 0..n {
            let current_index = mutable_numbers.iter().enumerate().find(|(_, (index, _))| *index == initial_index).unwrap().0;
            let step_forward = mutable_numbers[current_index].1.rem_euclid((n - 1) as i64) as usize;
            let next_index = (current_index + step_forward) % n;
            if current_index <= next_index {
                move_forward(&mut mutable_numbers, current_index, next_index);
            } else {
                move_backward(&mut mutable_numbers, current_index, next_index);
            }
        }    
    }
    let zero_index = mutable_numbers.iter().enumerate().find(|(_, (_, value))| *value == 0).unwrap().0;
    mutable_numbers[(zero_index + 1000) % n].1 + mutable_numbers[(zero_index + 2000) % n].1 + mutable_numbers[(zero_index + 3000) % n].1
}

// cargo test --bin day17 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = part2(&parse_input(&INPUT));
        assert_eq!(result, 1623178306);
    }

    const INPUT: &str ="1\n\
                        2\n\
                        -3\n\
                        3\n\
                        -2\n\
                        0\n\
                        4";
}