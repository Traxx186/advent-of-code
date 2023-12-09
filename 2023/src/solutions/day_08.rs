use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> String {
        "Haunted Wasteland".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_08");
        let mut steps = 0;
        let (instructions_str, nodes_lines) = input.split_once("\n\n").unwrap();
        
        let nodes: Vec<Node> = nodes_lines.lines()
            .map(|line| line.into())
            .collect_vec();
        
        let instructions: Vec<Instruction> = instructions_str.chars()
            .map(|c| c.into())
            .collect_vec();

        let mut current_node = nodes.iter()
            .find(|n| n.id == "AAA")
            .unwrap();

        for instruction in instructions.iter().cycle() {
            let target = match instruction {
                Instruction::Left => &current_node.left,
                Instruction::Right => &current_node.right,
            };

            steps += 1;
            if target == "ZZZ" {
                break;
            }

            current_node = nodes.iter()
                .find(|n| n.id == target.to_owned())
                .unwrap();
        }

        steps.to_string()
    }

    fn part_2(&self) -> String {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    id: String,
    left: String,
    right: String
}

impl Into<Node> for &str {
    fn into(self) -> Node {
        let (id, directions) = self.split_once(" = ").unwrap();
        let (left, right) = directions.trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();

        Node { 
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string()
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Left,
    Right
}

impl Into<Instruction> for char {
    fn into(self) -> Instruction {
        match self {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("{} invalid instruction", self)
        }
    }
}