use anyhow::Result;
use std::fs;
use num::integer::gcd;

#[derive(Debug)]
struct Map {
    faces: Vec<Face>,
    layout: Vec<Vec<usize>>,
}

impl Map {
    fn get_face_by_id(&self, id: usize) -> &Face {
        self.faces.iter().find(|face| face.id == id).unwrap()
    }
}

#[derive(Debug)]
struct Face {
    id: usize,
    map: Vec<Vec<Tile>>,
    offsets: (usize, usize),
}

#[derive(Debug)]
struct Teleport {
    edges: Vec<(usize, Direction, usize, Direction)>,
    size: usize,
}

impl Teleport {
    fn transport(&self, face_id: usize, current_dir: &Direction, x: usize, y: usize) ->  (usize, Direction, usize, usize) {
        let (next_face_id, next_dir) = self.edges.iter().find(|(id, dir, _, _ )| *id == face_id && dir == current_dir).map(|(_, _, id, dir)| (id, dir)).unwrap();
        match (current_dir, next_dir) {
            (Direction::Left, Direction::Right) => (*next_face_id, Direction::Left, x, self.size - 1),
            (Direction::Left, Direction::Left) => (*next_face_id, Direction::Right, self.size - 1 - x, 0),
            (Direction::Left, Direction::Up) => (*next_face_id, Direction::Down, 0, x),
            (Direction::Left, Direction::Down) => (*next_face_id, Direction::Up, self.size - 1, self.size - 1 - x),
            (Direction::Right, Direction::Left) => (*next_face_id, Direction::Right, x, 0),
            (Direction::Right, Direction::Right) => (*next_face_id, Direction::Left, self.size - 1 - x, self.size - 1),
            (Direction::Right, Direction::Up) => (*next_face_id, Direction::Down, 0, self.size - 1 - x),
            (Direction::Right, Direction::Down) => (*next_face_id, Direction::Up, self.size - 1, x),
            (Direction::Up, Direction::Down) => (*next_face_id, Direction::Up, self.size - 1, y),
            (Direction::Up, Direction::Left) => (*next_face_id, Direction::Right, y, 0),
            (Direction::Up, Direction::Right) => (*next_face_id, Direction::Left, self.size - 1 - y, self.size - 1),
            (Direction::Up, Direction::Up) => (*next_face_id, Direction::Down, 0, self.size - 1 - y),
            (Direction::Down, Direction::Up) => (*next_face_id, Direction::Down, 0, y),
            (Direction::Down, Direction::Left) => (*next_face_id, Direction::Right, self.size - 1 - y, 0),
            (Direction::Down, Direction::Right) => (*next_face_id, Direction::Left, y, self.size - 1),
            (Direction::Down, Direction::Down) => (*next_face_id, Direction::Up, self.size - 1, self.size - 1 - y),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    None,
    Clockwise,
    Upsidedown,
    CounterClockwise,
}

impl Rotation {
    fn reverse(&self) -> Rotation {
        match self {
            Rotation::None => Rotation::None,
            Rotation::Clockwise => Rotation::CounterClockwise,
            Rotation::Upsidedown => Rotation::Upsidedown,
            Rotation::CounterClockwise => Rotation::Clockwise,
        }
    }

    fn rotate(&self, dir: &Direction) -> Direction {
        match self {
            Rotation::None => 
                match dir {
                    Direction::Left => Direction::Left,
                    Direction::Up => Direction::Up,
                    Direction::Right => Direction::Right,
                    Direction::Down => Direction::Down,  
                },
            Rotation::Clockwise => 
                match dir {
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,  
                },
            Rotation::Upsidedown => 
                match dir {
                    Direction::Left => Direction::Right,
                    Direction::Up => Direction::Down,
                    Direction::Right => Direction::Left,
                    Direction::Down => Direction::Up,  
                },
            Rotation::CounterClockwise => 
                match dir {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,  
                },
        }
    }

    fn rotation(from: &Direction, to: &Direction) -> Rotation {
        match (from, to) {
            (Direction::Left, Direction::Left) => Rotation::None,
            (Direction::Left, Direction::Up) => Rotation::Clockwise,
            (Direction::Left, Direction::Right) => Rotation::Upsidedown,
            (Direction::Left, Direction::Down) => Rotation::CounterClockwise,
            (Direction::Up, Direction::Up) => Rotation::None,
            (Direction::Up, Direction::Right) => Rotation::Clockwise,
            (Direction::Up, Direction::Down) => Rotation::Upsidedown,
            (Direction::Up, Direction::Left) => Rotation::CounterClockwise,
            (Direction::Right, Direction::Right) => Rotation::None,
            (Direction::Right, Direction::Down) => Rotation::Clockwise,
            (Direction::Right, Direction::Left) => Rotation::Upsidedown,
            (Direction::Right, Direction::Up) => Rotation::CounterClockwise,
            (Direction::Down, Direction::Down) => Rotation::None,
            (Direction::Down, Direction::Left) => Rotation::Clockwise,
            (Direction::Down, Direction::Up) => Rotation::Upsidedown,
            (Direction::Down, Direction::Right) => Rotation::CounterClockwise,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Rotate(Rotation),
    Forward(usize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn move_from(&self, position: (usize, usize), size: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Left => if position.1 > 0 { Some((position.0, position.1 - 1)) } else { None },
            Direction::Right => if position.1 + 1 < size.1 { Some((position.0, position.1 + 1)) } else { None },
            Direction::Up => if position.0 > 0 { Some((position.0 - 1, position.1)) } else { None },
            Direction::Down => if position.0 + 1 < size.0 { Some((position.0 + 1, position.1)) } else { None },  
        }
    }

    fn move_around(&self, position: (usize, usize), size: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Left => (position.0, (position.1 + size.1 - 1) % size.1),
            Direction::Right => (position.0, (position.1 + 1) % size.1),
            Direction::Up => ((position.0 + size.0 - 1) % size.0, position.1),
            Direction::Down => ((position.0 + 1) % size.0, position.1),  
        }
    }

    fn score(&self) -> usize {
        match self {
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Down => 1,  
        }  
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,  
        }   
    }

    fn all_directions() -> Vec<Direction> {
        vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down]
    }
}

fn main() -> Result<()> {
    let (map, path): (Map, Vec<Instruction>) = parse_input(fs::read_to_string("day22.input")?.as_str());
    println!("{}", part1(&map, &path));
    println!("{}", part2(&map, &path));
    Ok(())
}

fn parse_input(input: &str) -> (Map, Vec<Instruction>) {
    input.split_once("\n\n")
        .map(|(map_str, path_str)| (parse_map(map_str), parse_instruction(path_str)))
        .unwrap()
}

fn parse_map(s: &str) -> Map {
    let raw_map: Vec<Vec<char>> = s.split("\n").map(|row| row.chars().collect::<Vec<char>>()).collect();
    let width = raw_map.iter().map(|v| v.len()).max().unwrap();
    let height = raw_map.len();
    let size = gcd(width, height);
    let n = width / size;
    let m = height / size;
    let mut faces: Vec<Face> = Vec::new();
    let mut layout = vec![vec![0; n]; m];
    let mut face_id = 0;
    for i in 0..m {
        for j in 0..n {
            if j * size < raw_map[i * size].len() && raw_map[i * size][j * size] != ' ' {
                let mut map = vec![vec![Tile::Open; size]; size];
                for x in 0..size {
                    for y in 0..size {
                        map[x][y] = match raw_map[i * size + x][j * size + y] {
                            '.' => Tile::Open,
                            '#' => Tile::Wall,
                            _ => panic!(),
                        };
                    }
                }
                face_id += 1;
                faces.push(Face { map: map, offsets: (i, j), id: face_id });
                layout[i][j] = face_id;
            }
        }
    }
    Map { faces, layout }
}

fn parse_instruction(s: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut number: usize = 0;
    for ch in s.chars() {
        match ch {
            'R' => {
                instructions.push(Instruction::Forward(number)); 
                instructions.push(Instruction::Rotate(Rotation::Clockwise));
                number = 0;
            },
            'L' => {
                instructions.push(Instruction::Forward(number)); 
                instructions.push(Instruction::Rotate(Rotation::CounterClockwise));
                number = 0;
            },
            d => number = number * 10 + d as usize - '0' as usize,
        }
    }
    instructions.push(Instruction::Forward(number));
    instructions
}

#[allow(dead_code)]
fn print_map(map: &Map) -> () {
    for face in &map.faces {
        let m = face.map.len(); 
        let n = face.map[0].len();
        println!("{:?} {:?}", face.id, face.offsets);
        for i in 0..m {
            for j in 0..n {
                match face.map[i][j] {
                    Tile::Open => print!("."),
                    Tile::Wall => print!("#"),
                }
            }
            println!();
        }
    }
}

fn part1(map: &Map, instructions: &Vec<Instruction>) -> usize {
    helper(map, instructions, teleport1(map))
}

fn teleport1(map: &Map) -> Teleport {
    let mut edges: Vec<(usize, Direction, usize, Direction)> = Vec::new();
    for face in &map.faces {
        for dir in Direction::all_directions() {
            let (mut i, mut j) = face.offsets;
            while {
                (i, j) = dir.move_around((i, j), (map.layout.len(), map.layout[0].len()));
                map.layout[i][j] == 0
            } {}
            edges.push((face.id, dir.clone(), map.layout[i][j], dir.opposite()))
        }
    }
    Teleport { edges: edges, size: map.faces[0].map.len() }
}

fn part2(map: &Map, instructions: &Vec<Instruction>) -> usize {
    helper(map, instructions, teleport2(map))
}

fn helper(map: &Map, instructions: &Vec<Instruction>, teleport: Teleport) -> usize {
    let (mut current_face_id, mut current_x, mut current_y, mut current_dir) = (1, 0, 0, Direction::Right);
    for instruction in instructions {
        match instruction {
            Instruction::Forward(steps) => {
                for _ in 0..*steps {
                    let current_face = map.get_face_by_id(current_face_id);
                    let next_move = current_dir.move_from((current_x, current_y), (current_face.map.len(), current_face.map[0].len()));
                    let (next_face_id, next_dir, next_x, next_y) = match next_move {
                        Some((x, y)) => (current_face_id, current_dir.clone(), x, y),
                        None => teleport.transport(current_face_id, &current_dir, current_x, current_y),
                    };
                    if map.get_face_by_id(next_face_id).map[next_x][next_y] == Tile::Wall {
                        break;
                    }
                    (current_face_id, current_x, current_y, current_dir) = (next_face_id, next_x, next_y, next_dir);
                };
            },
            Instruction::Rotate(rotation) => {
                current_dir = rotation.rotate(&current_dir);
            }
        }
    }
    let final_face = map.get_face_by_id(current_face_id);
    1000 * (current_x + 1 + final_face.offsets.0 * (final_face.map.len())) + 4 * (current_y + 1 + final_face.offsets.1 * (final_face.map[0].len())) + current_dir.score()
}

fn teleport2(map: &Map) -> Teleport {
    // standard cube:
    //  1
    // 234
    //  5
    //  6 
    let standard: Vec<(usize, Direction, usize, Direction)> = vec![
        (1, Direction::Left, 2, Direction::Up), (1, Direction::Right, 4, Direction::Up), (1, Direction::Up, 6, Direction::Down), (1, Direction::Down, 3, Direction::Up),
        (2, Direction::Left, 6, Direction::Left), (2, Direction::Right, 3, Direction::Left), (2, Direction::Up, 1, Direction::Left), (2, Direction::Down, 5, Direction::Left),
        (3, Direction::Left, 2, Direction::Right), (3, Direction::Right, 4, Direction::Left), (3, Direction::Up, 1, Direction::Down), (3, Direction::Down, 5, Direction::Up),
        (4, Direction::Left, 3, Direction::Right), (4, Direction::Right, 6, Direction::Right), (4, Direction::Up, 1, Direction::Right), (4, Direction::Down, 5, Direction::Right),
        (5, Direction::Left, 2, Direction::Down), (5, Direction::Right, 4, Direction::Down), (5, Direction::Up, 3, Direction::Down), (5, Direction::Down, 6, Direction::Up),
        (6, Direction::Left, 2, Direction::Left), (6, Direction::Right, 4, Direction::Right), (6, Direction::Up, 5, Direction::Down), (6, Direction::Down, 1, Direction::Up),
    ];
    // map from the face id of the cube to the face id of the standard cube
    let mut map_to_stardard_cube: Vec<(usize, usize, Rotation)> = vec![(1, 1, Rotation::None)];
    let mut start = 0;
    let mut end = 1;
    let mut visited = vec![false; 7];
    while start < end {
        for i in start..end {
            let (current_face_id, standard_face_id, current_rotation) = map_to_stardard_cube[i];
            visited[current_face_id] = true;
            for current_dir in Direction::all_directions() {
                if let Some((next_x, next_y)) = current_dir.move_from(map.get_face_by_id(current_face_id).offsets, (map.layout.len(), map.layout[0].len())) {
                    if map.layout[next_x][next_y] != 0 && !visited[map.layout[next_x][next_y]] {
                        let next_face_id = map.layout[next_x][next_y];
                        let (next_standard_face_id, next_standard_face_dir) = standard
                            .iter()
                            .find(|(id, dir, _, _ )| *id == standard_face_id && *dir == current_rotation.rotate(&current_dir))
                            .map(|(_, _, id, dir)| (id, dir))
                            .unwrap();
                        map_to_stardard_cube.push((next_face_id, *next_standard_face_id, Rotation::rotation(&current_dir.opposite(), next_standard_face_dir)));
                    }
                }
            }
        }
        start = end;
        end = map_to_stardard_cube.len();
    }

    let mut edges: Vec<(usize, Direction, usize, Direction)> = Vec::new();
    for face in &map.faces {
        for current_dir in Direction::all_directions() {
            let (standard_face_id, forward_rotation) = map_to_stardard_cube.iter().find(|(id, _, _)| *id == face.id).map(|(_, sid, rotation)| (sid, rotation)).unwrap();
            let (next_stardard_face_id, next_standard_dir) = standard
                .iter()
                .find(|(id, dir, _, _)| *id == *standard_face_id && *dir == forward_rotation.rotate(&current_dir))
                .map(|(_, _, id, dir)| (id, dir))
                .unwrap();
            let (next_face_id, reverse_rotation) = map_to_stardard_cube.iter().find(|(_, sid, _)| *sid == *next_stardard_face_id).map(|(id, _, rotation)| (id, rotation)).unwrap();
            edges.push((face.id, current_dir, *next_face_id, reverse_rotation.reverse().rotate(next_standard_dir)));
        }
    }
    Teleport { edges: edges, size: map.faces[0].map.len() }
}

// cargo test --bin day22 -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (map, path) = parse_input(&INPUT);
        let result = part1(&map, &path);
        assert_eq!(result, 6032);
    }

    #[test]
    fn test2() {
        let (map, path) = parse_input(&INPUT);
        let result = part2(&map, &path);
        assert_eq!(result, 5031);
    }

    const INPUT: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
}