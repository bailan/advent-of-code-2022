use anyhow::Result;
use std::fs;
use std::collections::HashMap;
use Expression::*;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Operator {
    Add,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Expression {
    Number(i64),
    Variable(String),
    Arithmetic(Box<Expression>, Box<Expression>, Operator),
}

impl Expression {
    fn get_number(&self) -> i64 {
        match &self {
            Number(number) => *number,
            _ => panic!(),
        }
    }
}

fn main() -> Result<()> {
    let mut assignments: HashMap<Expression, Expression> = parse_input(fs::read_to_string("day21.input")?.as_str());
    println!("{}", part1(&assignments));
    println!("{}", part2(&mut assignments));
    Ok(())
}

fn parse_input(input: &str) -> HashMap<Expression, Expression> {
    input.split("\n").map(|s| parse_line(s)).collect()
}

fn parse_line(s: &str) -> (Expression, Expression) {
    let (variable, exp_string) = s.split_once(": ").unwrap();
    let expression = match exp_string.parse::<i64>() {
        Ok(number) => Number(number),
        Err(_) => {
            let splits: Vec<&str> = exp_string.split(" ").collect();
            let operator = match splits[1] {
                "+" => Operator::Add,
                "-" => Operator::Minus,
                "*" => Operator::Multiply,
                "/" => Operator::Divide,
                _ => panic!(),
            };
            Arithmetic(Box::new(Variable(splits[0].to_string())), Box::new(Variable(splits[2].to_string())), operator)
        },
    };
    (Variable(variable.to_string()), expression)
}

fn part1(assignments: &HashMap<Expression, Expression>) -> i64 {
    eval(assignments, &Variable(String::from("root"))).get_number()
}

fn eval(assignments: &HashMap<Expression, Expression>, expression: &Expression) -> Expression {
    match expression {
        Number(number) => Number(*number),
        Variable(variable) => match assignments.get(expression)  {
                Some(exp) => eval(assignments, exp),
                None => Variable(variable.to_string())
            },
        Arithmetic(boxed_expression1, boxed_expression2, operator) => {
            let expression1 = eval(assignments, &**boxed_expression1);
            let expression2 = eval(assignments, &**boxed_expression2);
            match (&expression1, &expression2, operator) {
                (Number(number1), Number(number2), Operator::Add) => Number(number1 + number2),
                (Number(number1), Number(number2), Operator::Minus) => Number(number1 - number2),
                (Number(number1), Number(number2), Operator::Multiply) => Number(number1 * number2),
                (Number(number1), Number(number2), Operator::Divide) => Number(number1 / number2),
                _ => Arithmetic(Box::new(expression1), Box::new(expression2), operator.clone()),
            }
        },
    }
}

fn part2(assignments: &mut HashMap<Expression, Expression>) -> i64 {
    assignments.remove(&Variable(String::from("humn")));
    let root_assignment = assignments.get(&Variable(String::from("root"))).unwrap();                        
    let (expression1, expression2) = match root_assignment {
        Arithmetic(boxed_expression1, boxed_expression2, _) => (&**boxed_expression1, &**boxed_expression2),
        _ => panic!(),
    };
    solve(&eval(assignments, expression1), &eval(assignments, expression2))
}

fn solve(left: &Expression, right: &Expression) -> i64 {
    match (left, right) {
        (Arithmetic(boxed_expression1, boxed_expression2, operator), Number(number)) => 
            match (&**boxed_expression1, &**boxed_expression2) {
                (_, Number(inner)) => 
                    match operator {
                        Operator::Add => solve(&**boxed_expression1, &Number(number - inner)),
                        Operator::Minus => solve(&**boxed_expression1, &Number(number + inner)),
                        Operator::Multiply => solve(&**boxed_expression1, &Number(number / inner)),
                        Operator::Divide => solve(&**boxed_expression1, &Number(number * inner)),
                    }
                (Number(inner), _) => {
                    match operator {
                        Operator::Add => solve(&**boxed_expression2, &Number(number - inner)),
                        Operator::Minus => solve(&**boxed_expression2, &Number(inner - number)),
                        Operator::Multiply => solve(&**boxed_expression2, &Number(number / inner)),
                        Operator::Divide => solve(&**boxed_expression2, &Number(inner / number)),
                    }
                },
                _ => {println!("{:?}", left); todo!()},
            },
        (Variable(_), Number(number)) => *number,
        _ => solve(right, left),
    }
}

// cargo test --bin 21 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT));
        assert_eq!(result, 152);
    }

    #[test]
    fn test2() {
        let result = part2(&mut parse_input(&INPUT));
        assert_eq!(result, 301);
    }

    const INPUT: &str ="root: pppw + sjmn\n\
                        dbpl: 5\n\
                        cczh: sllz + lgvd\n\
                        zczc: 2\n\
                        ptdq: humn - dvpt\n\
                        dvpt: 3\n\
                        lfqf: 4\n\
                        humn: 5\n\
                        ljgn: 2\n\
                        sjmn: drzm * dbpl\n\
                        sllz: 4\n\
                        pppw: cczh / lfqf\n\
                        lgvd: ljgn * ptdq\n\
                        drzm: hmdt - zczc\n\
                        hmdt: 32";                
}