use anyhow::Result;
use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: fn(i64) -> i64,
    divisor: i64,
    true_index: usize,
    false_index: usize,
}

impl Monkey {
    fn of(items: Vec<i64>, operation: fn(i64) -> i64, divisor: i64, true_index: usize, false_index: usize) -> Monkey {
        Monkey { items, operation, divisor, true_index, false_index }
    }
}

fn main() -> Result<()> {
    let monkeys = parse_input(fs::read_to_string("day11.input")?.as_str());
    // let monkeys = vec![
    //     Monkey::of(Vec::from([98, 89, 52]), |x| x * 2, 5, 6, 1),
    //     Monkey::of(Vec::from([57, 95, 80, 92, 57, 78]), |x| x * 13, 2, 2, 6),
    //     Monkey::of(Vec::from([82, 74, 97, 75, 51, 92, 83]), |x| x + 5, 19, 7, 5),
    //     Monkey::of(Vec::from([97, 88, 51, 68, 76]), |x| x + 6, 7, 0, 4),
    //     Monkey::of(Vec::from([63]), |x| x + 1, 17, 0, 1),
    //     Monkey::of(Vec::from([94, 91, 51, 63]), |x| x + 4, 13, 4, 3),
    //     Monkey::of(Vec::from([61, 54, 94, 71, 74, 68, 98, 83]), |x| x + 2, 3, 2, 7),
    //     Monkey::of(Vec::from([90, 56]), |x| x * x, 11, 3, 5),
    // ];
    println!("{}", part1(&monkeys));
    println!("{}", part2(&monkeys));
    Ok(())
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n")
    .map(|line| parse_monkey(line))
    .collect()
}

fn parse_monkey(money_string: &str) -> Monkey {
    let lines: Vec<&str> = money_string.split("\n").collect();
    let items: Vec<i64> = lines[1].strip_prefix("  Starting items: ").unwrap().split(", ").map(|x| x.parse::<i64>().unwrap()).collect();
    let operation = parse_operation(lines[2].strip_prefix("  Operation: new = ").unwrap());
    let divisor = lines[3].strip_prefix("  Test: divisible by ").unwrap().parse::<i64>().unwrap();
    let true_index = lines[4].strip_prefix("    If true: throw to monkey ").unwrap().parse::<usize>().unwrap();
    let false_index = lines[5].strip_prefix("    If false: throw to monkey ").unwrap().parse::<usize>().unwrap();
    Monkey::of(items, operation, divisor, true_index, false_index)
}

fn parse_operation(s: &str) -> fn(i64) -> i64 {
    println!("{s}");
    let mut iter = s.splitn(3, " ");
    iter.next();
    let operator = iter.next().unwrap();
    let right = iter.next().unwrap();
    match (operator, right) {
        ("+", _) => {let r = right.parse::<i64>().unwrap(); |x| x + r},
        ("*", _) => {let r = right.parse::<i64>().unwrap(); |x| x * r},
        ("*", "old") => |x| x * x,
        _ => panic!(),
    }
}

fn part1(monkeys: &Vec<Monkey>) -> i64 {
    helper(monkeys, 20, &|x| x / 3)
}

fn part2(monkeys: &Vec<Monkey>) -> i64 {
    let modulus: i64 = monkeys.iter().map(|monkey| monkey.divisor).product();
    helper(monkeys, 10000, &|x| x % modulus)
}

fn helper(monkeys: &Vec<Monkey>, round: u32, relief_function: &dyn Fn(i64) -> i64) -> i64 {
    let number_of_moneys = monkeys.len();
    let mut current_round_items: Vec<Vec<i64>> = monkeys.iter().map(|monkey| monkey.items.clone()).collect();
    let mut inspected_counts = vec![0; number_of_moneys];
    for _ in 0..round {
        for monkey_index in 0..number_of_moneys {
            let mut next_round_items = current_round_items.clone();
            next_round_items[monkey_index] = Vec::new();
            let items_of_current_monkey = &current_round_items[monkey_index];
            inspected_counts[monkey_index] += items_of_current_monkey.len();
            for item in items_of_current_monkey {
                let new_item = relief_function((monkeys[monkey_index].operation)(*item));
                if new_item % monkeys[monkey_index].divisor == 0 {
                    next_round_items[monkeys[monkey_index].true_index].push(new_item);
                } else {
                    next_round_items[monkeys[monkey_index].false_index].push(new_item);
                }
            }
            current_round_items = next_round_items.clone();
        }
    }
    inspected_counts.sort();
    inspected_counts.reverse();
    inspected_counts[0] as i64 * inspected_counts[1] as i64
}


// cargo test --bin day10 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test1() {
        let monkeys = vec![
            Monkey::of(Vec::from([79, 98]), |x| x * 19, 23, 2, 3),
            Monkey::of(Vec::from([54, 65, 75, 74]), |x| x + 6, 19, 2, 0),
            Monkey::of(Vec::from([79, 60, 97]), |x| x * x, 13, 1, 3),
            Monkey::of(Vec::from([74]), |x| x + 3, 17, 0, 1),
        ];
        let result = part1(&monkeys);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test2() {
        let monkeys = vec![
            Monkey::of(Vec::from([79, 98]), |x| x * 19, 23, 2, 3),
            Monkey::of(Vec::from([54, 65, 75, 74]), |x| x + 6, 19, 2, 0),
            Monkey::of(Vec::from([79, 60, 97]), |x| x * x, 13, 1, 3),
            Monkey::of(Vec::from([74]), |x| x + 3, 17, 0, 1),
        ];
        let result = part2(&monkeys);
        assert_eq!(result, 2713310158);
    }

    const INPUT: &str = "Monkey 0:\n\
                        Starting items: 79, 98\n\
                        Operation: new = old * 19\n\
                        Test: divisible by 23\n\
                        If true: throw to monkey 2\n\
                        If false: throw to monkey 3\n\
                        \n\
                        Monkey 1:\n\
                            Starting items: 54, 65, 75, 74\n\
                            Operation: new = old + 6\n\
                            Test: divisible by 19\n\
                            If true: throw to monkey 2\n\
                            If false: throw to monkey 0\n\
                        \n\
                        Monkey 2:\n\
                            Starting items: 79, 60, 97\n\
                            Operation: new = old * old\n\
                            Test: divisible by 13\n\
                            If true: throw to monkey 1\n\
                            If false: throw to monkey 3\n\
                        \n\
                        Monkey 3:\n\
                            Starting items: 74\n\
                            Operation: new = old + 3\n\
                            Test: divisible by 17\n\
                            If true: throw to monkey 0\n\
                            If false: throw to monkey 1";
}