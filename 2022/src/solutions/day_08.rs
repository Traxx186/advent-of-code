use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day08 {}

impl Solution for Day08 {

    fn name(&self) -> String {
        "Treetop Tree House".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_08");
        let trees = input.lines()
            .map(|line| line.chars().collect_vec())
            .map(|tree| tree.into_iter().map(|t| t.to_digit(10).unwrap() as i32).collect_vec())
            .collect_vec();

        let mut visible = trees.iter()
            .map(|row| row.iter().map(|_| false).collect_vec())
            .collect_vec();

        let vissible_trees = count_vissible_trees(trees, &mut visible);

        vissible_trees.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_08");
        let trees = input.lines()
            .map(|line| line.chars().collect_vec())
            .map(|tree| tree.into_iter().map(|t| t.to_digit(10).unwrap() as i32).collect_vec())
            .collect_vec();

        let senic_score = count_view_score(trees);
        senic_score.to_string()
    }
}

fn count_vissible_trees(trees_lines: Vec<Vec<i32>>, visible: &mut Vec<Vec<bool>>) -> usize {
    let mut vissible_trees = 0;
    
    let height = trees_lines.len();
    let width = trees_lines[0].len();

    for (y, row) in trees_lines.iter().enumerate() {
        let mut max_height = -1;
        for (x, tree) in row.iter().enumerate() {
            if !tree_vissible(tree, &max_height) {
                continue;                    
            }
            
            max_height = *tree;
            let this_visited = &mut visible[y][x];
            if !*this_visited {
                *this_visited = true;
                vissible_trees += 1;
            }
        }

        max_height = -1;
        for x in 0..row.len() {
            let tree = row[width - 1 - x];
            if !tree_vissible(&tree, &max_height) {
                continue;                    
            }
            
            max_height = tree;
            let this_visited = &mut visible[y][width - 1 - x];
            if !*this_visited {
                *this_visited = true;
                vissible_trees += 1;
            }
        }
    }

    for x in 0..width {
        let mut max_height = -1;
        for y in 0..height {
            let tree = trees_lines[y][x];
            if !tree_vissible(&tree, &max_height) {
                continue;                    
            }
        
            max_height = tree;
            let this_visited = &mut visible[y][x];
            if !*this_visited {
                *this_visited = true;
                vissible_trees += 1;                
            }
        }

        let mut max_height = -1;
        for y in 0..height {
            let tree = trees_lines[height - 1 - y][x];
            if !tree_vissible(&tree, &max_height) {
                continue;                    
            }

            max_height = tree;
            let this_visited = &mut visible[height - 1 - y][x];
            if !*this_visited {
                *this_visited = true;
                vissible_trees += 1;
            }
        }
    }

    vissible_trees
}

fn tree_vissible(a: &i32, b: &i32) -> bool {
    return a > b;
}

fn count_view_score(tree_lines: Vec<Vec<i32>>) -> u32 {
    let height = tree_lines.len();
    let width = tree_lines[0].len();

    tree_lines.iter() .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &height)| (y, x, height)))
            .map(|(y, x, tree_height)| {
        let score = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ].iter().map(|(dy, dx)| {
            let (mut y, mut x) = (y as isize, x as isize);
            let mut distance = 0;
            let mut altitude = -1;
            loop {
                y += dy;
                x += dx;
                if y < 0 || x < 0 || y as usize >= height || x as usize >= width {
                    break;
                }
                distance += 1;
                let this_height = tree_lines[y as usize][x as usize];
                if this_height > altitude {
                    if this_height >= tree_height {
                        break;
                    }
                    altitude = this_height;
                }
            }
            distance
        }).product::<u32>();
        score
    }).max().unwrap()
}