use anyhow::Result;
use std::fs;
use std::slice::Iter;
use std::collections::LinkedList;

#[derive(Debug)]
struct File {
    _name: String,
    size: u32,
}

#[derive(Debug)]
struct Directory {
    files: LinkedList<File>,
    dirs: LinkedList<Directory>,
}

fn main() -> Result<()> {
    let root: Directory = parse_file_system(fs::read_to_string("day7.input")?.as_str());
    let size_of_directories: LinkedList<u32> = get_directory_size(&root);
    println!("{}", part1(&size_of_directories));
    println!("{}", part2(&size_of_directories));
    Ok(())
}

fn part1(size_of_directories: &LinkedList<u32>) -> u32 {
    size_of_directories.iter().filter(|size| **size < 100000).sum()
}

fn part2(size_of_directories: &LinkedList<u32>) -> u32 {
    let mut sorted_directory_sizes: Vec<u32> = size_of_directories.iter().cloned().collect();
    sorted_directory_sizes.sort();
    let total: u32 = *sorted_directory_sizes.last().unwrap();
    *sorted_directory_sizes.iter().find(|size| total - **size < 40000000).unwrap()
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    Parent,
    Root,
    List,
}

fn parse_file_system(terminal_output: &str) -> Directory {
    parse_line(&mut terminal_output.split("\n").collect::<Vec<&str>>().iter())
}

fn parse_line(cursor: &mut Iter<'_, &str>) -> Directory {
    let mut files: LinkedList<File> = LinkedList::new();
    let mut dirs: LinkedList<Directory> = LinkedList::new();
    while let Some(line) = cursor.next() {
        if line.starts_with("$") {
            match parse_command(line.split_once(" ").unwrap().1) {
                Command::Root => continue,
                Command::Parent => break,
                Command::ChangeDirectory(_) => dirs.push_back(parse_line(cursor)),
                Command::List => continue,
            }
        } else {
            match parse_file(line) {
                Some(file) => files.push_back(file),
                None => continue,
            }
        }
    }
    Directory {files: files, dirs: dirs}
}

fn parse_command(command: &str) -> Command {
    match command.split_once(" ") {
        Some(("cd", "/")) => Command::Root,
        Some(("cd", "..")) => Command::Parent,
        Some(("cd", dir)) => Command::ChangeDirectory(dir.to_string()),
        None => Command::List,
        Some((_, _)) => panic!("unrecorgnized command {command}")
    }
}

fn parse_file(list_line: &str) -> Option<File> {
    match list_line.split_once(" ") {
        Some(("dir", _)) => None,
        Some((size, name)) => Some(File {_name: name.to_string(), size: size.parse::<u32>().unwrap()}),
        _ => panic!("unrecorgnized list result {list_line}")
    }
}

fn get_directory_size(current: &Directory) -> LinkedList<u32> {
    let size_of_files: u32 = current.files.iter().map(|file| file.size).sum();
    let directories: Vec<LinkedList<u32>> = current.dirs.iter().map(|dir| get_directory_size(dir)).collect();
    let size_of_directories: u32 = directories.iter().filter_map(|result| result.front()).sum();
    let mut result: LinkedList<u32> = LinkedList::new();
    result.push_back(size_of_files + size_of_directories); // put the size of the current directory at the front
    for directory in directories.iter().flatten() {
        result.push_back(*directory);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TERMINAL_OUTPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test1() {
        let result: u32 = part1(&get_directory_size(&parse_file_system(TERMINAL_OUTPUT)));
        assert_eq!(result, 95437);
    }

    #[test]
    fn test2() {
        let result: u32 = part2(&get_directory_size(&parse_file_system(TERMINAL_OUTPUT)));
        assert_eq!(result, 24933642);
    }
}