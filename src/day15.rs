use anyhow::Result;
use std::fs;
use std::cmp::Ordering;
use std::cmp::min;
use std::cmp::max;
use std::collections::HashSet;
use regex::Regex;


#[derive(Debug)]
struct Intervals {
    intervals: Vec<Interval>,
}

impl Intervals {
    fn from(intervals: Vec<Interval>) -> Intervals {
        let mut sorted_intervals = intervals;
        sorted_intervals.sort();
        let mut merged_intervals: Vec<Interval> = Vec::new();
        for interval in sorted_intervals.clone() {
            let last: Option<Interval> = merged_intervals.pop();
            match last {
                Some(last_interval) => 
                    if last_interval.overlap(&interval) {
                        merged_intervals.push(last_interval.merge(&interval));
                    } else {
                        merged_intervals.push(last_interval);
                        merged_intervals.push(interval);
                    }
                None => merged_intervals.push(interval),
            };
        }
        Intervals { intervals: merged_intervals }
    }

    fn len(&self) -> usize {
        self.intervals.iter().map(|interval| interval.len()).sum()
    }

    fn contain(&self, point: i32) -> bool {
        self.intervals.iter().any(|interval| interval.contains(point))
    }

    fn intersect(&self, interval: Interval) -> Intervals {
        Intervals::from(self.intervals
            .iter()
            .filter_map(|i| i.intersect(&interval))
            .collect())
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
struct Interval {
    start: i32,
    end: i32,
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start).then(self.end.cmp(&other.end))
    }
}

impl Interval {
    fn overlap(&self, other: &Self) -> bool {
        max(self.start, other.start) <= min(self.end, other.end)
    }

    fn merge(&self, other: &Self) -> Interval {
        Interval { start: min(self.start, other.start), end: max(self.end, other.end) }
    }

    fn len(&self) -> usize {
        (self.end - self.start + 1) as usize
    }

    fn contain(&self, point: i32) -> bool {
        point >= self.start && point <= self.end
    }

    fn intersect(&self, other: &Self) -> Option<Interval> {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        if start <= end {
            Some(Interval { start, end })
        } else {
            None
        }
    }
}

fn main() -> Result<()> {
    let lines: Vec<((i32, i32), (i32, i32))> = parse_input(fs::read_to_string("day15.input")?.as_str());
    println!("{}", part1(&lines, 2000000));
    println!("{}", part2(&lines, 4000000));
    Ok(())
}

fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input.split("\n")
      .map(|line| parse_line(line))
      .collect()
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let re = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)")
      .expect("invalid regex");
    let caps = re.captures(line).expect("parse {line}");
    ((caps["sensor_x"].parse::<i32>().unwrap(), caps["sensor_y"].parse::<i32>().unwrap()), 
        (caps["beacon_x"].parse::<i32>().unwrap(), caps["beacon_y"].parse::<i32>().unwrap()))
}

fn intersaction(center: (i32, i32), distance: i32, line_y: i32) -> Option<Interval> {
    let x_offset = distance - (center.1 - line_y).abs();
    if x_offset > 0 {
        Some(Interval { start: center.0 - x_offset, end: center.0 + x_offset })
    } else {
        None
    }
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn part1(data: &Vec<((i32, i32), (i32, i32))>, line_y: i32) -> usize {
    let intervals: Intervals = Intervals::from(
        data.iter()
            .map(|(sensor, beacon)| ((sensor.0, sensor.1), distance(*sensor, *beacon)))
            .filter_map(|(sensor, distance)| intersaction(sensor, distance, line_y))
            .collect());
    let beacon_on_line: HashSet<i32> = data.iter()
      .filter(|(_, beacon)| beacon.1 == line_y)
      .map(|(_, beacon)| beacon.0)
      .collect();
    intervals.len() - beacon_on_line.iter().filter(|point| intervals.contains(**point)).count()
}

fn part2(data: &Vec<((i32, i32), (i32, i32))>, size: usize) -> usize {
    let mut length = 0;
    for line_y in 0..(size+1) {
        let intervals: Intervals = Intervals::from(
            data.iter()
                .map(|(sensor, beacon)| ((sensor.0, sensor.1), distance(*sensor, *beacon)))
                .filter_map(|(sensor, distance)| intersaction(sensor, distance, line_y as i32))
                .collect());
        if intervals.intersect(Interval { start: 0, end: size as i32}).len() != size + 1 {
            length = (intervals.intervals[0].end + 1) as usize * 4000000 + line_y;
            break;
        }
    }
    length
}

// cargo test --bin day14 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test1() {
        let result = part1(&parse_input(&INPUT), 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn test2() {
        let result = part2(&parse_input(&INPUT), 20);
        assert_eq!(result, 56000011);
    }

    const INPUT: &str ="Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
                        Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
                        Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
                        Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
                        Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
                        Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
                        Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
                        Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
                        Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
                        Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
                        Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
                        Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
                        Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
                        Sensor at x=20, y=1: closest beacon is at x=15, y=3";
}