use anyhow::Result;
use std::fs;

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn contains(&self, another: &Assignment) -> bool {
        self.start <= another.start && self.end >= another.end
    }

    fn overlaps(&self, another: &Assignment) -> bool {
        if self.start <= another.start {
            another.start <= self.end
        } else {
            self.start <= another.end
        }
    }

    fn of(start: u32, end: u32) -> Assignment {
        Assignment { start: start, end: end }
    }

    fn parse(pair_string: &str) -> Assignment {
        pair_string.split_once("-")
          .map(|(start, end)| (start.parse::<u32>().expect("integer {start}"), end.parse::<u32>().expect("integer {end}")))
          .map(|(start, end)| Assignment::of(start, end))
          .expect("split {pair_string} with '-'")
    }
}

fn main() -> Result<()> {
    let assignments: Vec<(Assignment, Assignment)> = fs::read_to_string("day4.input")?
        .split("\n")
        .map(|line| line.split_once(",").expect("split {line} with ','"))
        .map(|(first, second)| (Assignment::parse(first), Assignment::parse(second)))
        .collect();
    let part1: usize = assignments.iter()
      .filter(|(first, second)| first.contains(second) || second.contains(first))
      .count();
    println!("{part1}");
    
    let part2: usize = assignments.iter()
      .filter(|(first, second)| first.overlaps(second))
      .count();
    println!("{part2}");
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        assert_eq!(Assignment::of(1, 1).overlaps(&Assignment::of(1, 1)), true);
        assert_eq!(Assignment::of(1, 2).overlaps(&Assignment::of(2, 2)), true);
        assert_eq!(Assignment::of(2, 2).overlaps(&Assignment::of(1, 2)), true);
        assert_eq!(Assignment::of(1, 4).overlaps(&Assignment::of(2, 3)), true);
        assert_eq!(Assignment::of(2, 3).overlaps(&Assignment::of(1, 4)), true);

        assert_eq!(Assignment::of(1, 1).overlaps(&Assignment::of(2, 2)), false);
        assert_eq!(Assignment::of(2, 2).overlaps(&Assignment::of(1, 1)), false);
        assert_eq!(Assignment::of(1, 2).overlaps(&Assignment::of(3, 4)), false);
        assert_eq!(Assignment::of(3, 4).overlaps(&Assignment::of(1, 2)), false);
    }
}