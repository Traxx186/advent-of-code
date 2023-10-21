use aoc_common::{Solution, load};
use itertools::Itertools;

const TOTAL_STORAGE_SIZE: usize = 70_000_000;
const MIN_REQUIRED_STORAGE_SPACE: usize = 30_000_000;

pub struct Day07 {}

#[derive(Clone, Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    dirs: Vec<Directory>,
    total_size: usize,
}

impl Solution for Day07 {

    fn name(&self) -> String {
        "No Space Left On Device".to_owned()
    }

    fn part_1(&self) -> String {
        let file_structure = parse_input(load("day_07"));
        let total_size: usize = get_total_sizes(&file_structure).into_iter()
            .filter(|size| size <= &100000)
            .sum();

        total_size.to_string()
    }

    fn part_2(&self) -> String {
        let file_structure = parse_input(load("day_07"));
        let mut total_sizes = get_total_sizes(&file_structure);
        let free_space_required = MIN_REQUIRED_STORAGE_SPACE - (TOTAL_STORAGE_SIZE - file_structure.total_size);

        total_sizes.sort();
        total_sizes.into_iter()
            .skip_while(|&size| size < free_space_required)
            .next()
            .unwrap()
            .to_string()
    }
}

impl Directory {
    fn new(name: String) -> Self {
        Directory {
            name,
            files: vec![], 
            dirs: vec![], 
            total_size: 0,
        }
    }
}

fn parse_input(input: String) -> Directory {
    let mut root = Directory::new("".to_string());
    let mut path: Vec<String> = vec![]; 

    for line in input.lines().skip(1) {
        if line.starts_with("$ cd") {
            let dir = &line[5..];
            if dir == ".." {
                path.pop();
            } else {
                path.push(dir.to_string())
            }
        } else if line.starts_with("$ ls") {   
        } else if line.starts_with("dir ") {
            parse_command(
                &mut root,
                &path, 
                |dir| dir.dirs.push(Directory::new(line[4..].to_string())))
        } else {
            parse_command(&mut root,
                &path, 
                |dir| dir.files.push(get_file_from_line(line)))
        }
    }

    calculate_dir_size(&mut root);

    root
}

fn parse_command<F>(dir: &mut Directory, 
    path: &[String], 
    operation: F
) where F: Fn(&mut Directory) {
    if path.is_empty() {
        operation(dir);
        return;
    }

    let child_dir = dir.dirs.iter_mut()
        .find(|d| d.name == path[0])
        .unwrap();

    parse_command(child_dir, &path[1..], operation);   
}

fn get_file_from_line(line: &str) -> File {
    let parts: (&str, &str) = line.split(" ").collect_tuple().unwrap();

    File { size: parts.0.parse::<usize>().unwrap() }
}

fn calculate_dir_size(dir: &mut Directory) -> usize {
    let total_dir_size: usize = dir.dirs.iter_mut()
        .map(calculate_dir_size)
        .sum(); 

    let total_file_size: usize = dir.files.clone().into_iter()
        .map(|f| f.size)
        .sum();

    dir.total_size = total_dir_size + total_file_size;

    dir.total_size
}

fn get_total_sizes(dir: &Directory) -> Vec<usize> {
    let mut sizes = vec![];
    get_total_size(dir, &mut sizes);

    sizes
}

fn get_total_size(dir: &Directory, mut sizes: &mut Vec<usize>) {
    for directory in &dir.dirs {
        get_total_size(&directory, &mut sizes)
    }

    sizes.push(dir.total_size)
}