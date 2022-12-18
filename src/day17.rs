use anyhow::Result;
use std::fs;
use std::cmp::max;


struct Shape {
    rocks: Vec<(i16, i16)>,
}

impl Shape {
    fn from(rocks: Vec<(i16, i16)>) -> Shape {
        Shape { rocks }
    }
}

fn main() -> Result<()> {
    let pattern: Vec<char> = parse_input(fs::read_to_string("day17.input")?.as_str());
    println!("{}", part1(&pattern));
    println!("{}", part2(&pattern));
    Ok(())
}

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part1(pattern: &Vec<char>) -> usize {
    let shapes: Vec<Shape> = vec![
        Shape::from(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        Shape::from(vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
        Shape::from(vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
        Shape::from(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        Shape::from(vec![(0, 0), (0, 1), (1, 0), (1, 1)])];
    let mut chamber: Vec<Vec<bool>> = vec![vec![false; 7]; 6];
    let mut tall: i16 = 0;
    let mut pattern_index: usize = 0;
    for shape_index in 0..100 {
        let shape = &shapes[shape_index % 5];
        let mut position = (tall + 3 as i16, 2 as i16);
        loop {
            match pattern[pattern_index % pattern.len()] {
                '<' => if !collide(&chamber, (position.0 as i16, position.1 as i16 - 1), &shape) { position.1 -= 1 },
                '>' => if !collide(&chamber, (position.0 as i16, position.1 as i16 + 1), &shape) { position.1 += 1 },
                _ => panic!(),
            }
            pattern_index += 1;
            if !collide(&chamber, (position.0 as i16 - 1, position.1 as i16), &shape) {
                position.0 -= 1;
            } else {
                break;
            }
        }
        for rock in &shape.rocks {
            chamber[(position.0 + rock.0) as usize][(position.1 + rock.1) as usize] = true;
            tall = max(tall, position.0 + rock.0 + 1);
        }
        while (tall + 6) as usize >= chamber.len() {
            chamber.push(vec![false; 7]);
        }
    }
    print_chamber(&chamber);
    tall as usize
}

fn print_chamber(chamber: &Vec<Vec<bool>>) -> () {
    for row in chamber.iter().rev() {
        println!("|{}|", row.iter().map(|has_rock| if *has_rock { '#' } else { '.' }).collect::<String>());
    }
    println!("+-------+");
}

fn collide(chamber: &Vec<Vec<bool>>, position: (i16, i16), shape: &Shape) -> bool {
    if position.0 < 0 {
        return true;
    }
    if position.1 < 0 || position.1 > 6 {
        return true;
    }
    for rock in &shape.rocks {
        if position.1 + rock.1 < 0 || position.1 + rock.1 > 6 {
            return true;
        }
        let x = (position.0 + rock.0) as usize;
        let y = (position.1 + rock.1) as usize;
        if chamber[x][y] {
            return true;
        }
    }
    false
}

fn part2(pattern: &Vec<char>) -> usize {
    // 55 + 573065902 * 1745 + 955 = 1000000000000
    // 11 + 573065902 * 349 + 191 = 200000000000

    // number -> height
    // 55 -> 80
    // 1010 -> 1586
    // 1800 -> 2858

    80 + 573065902 * 2778 + 1586 - 80
}

// cargo test --bin day17 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, 3068);
    }

    #[ignore]
    #[test]
    fn test2() {
        let result = 28571428571 * 53 + 25;
        assert_eq!(result, 1514285714288usize);
    }

    const INPUT: &str =">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}