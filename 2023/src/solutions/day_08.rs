use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> String {
        "Haunted Wasteland".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_08");
        let (instructions_str, nodes_lines) = input.split_once("\n\n").unwrap();
        
        let nodes: Vec<Node> = nodes_lines.lines()
            .map(|line| line.into())
            .collect_vec();
        
        let instructions: Vec<Instruction> = instructions_str.chars()
            .map(|c| c.into())
            .collect_vec();

        let start_node = nodes.iter()
            .find(|n| n.id == "AAA")
            .unwrap();
        
        count_steps(&instructions, start_node, &nodes, |s| s == "AAA")
            .to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_08");
        let (instructions_str, nodes_lines) = input.split_once("\n\n").unwrap();
        
        let nodes: Vec<Node> = nodes_lines.lines()
            .map(|line| line.into())
            .collect_vec();
        
        let instructions: Vec<Instruction> = instructions_str.chars()
            .map(|c| c.into())
            .collect_vec();

        let steps = nodes.iter()
            .filter_map(|node| 
                node.id.ends_with('A')
                    .then_some(node)
                    .map(|n| count_steps(&instructions, n, &nodes, |s| s.ends_with('Z')))
            )
            .collect_vec();
        
        steps.iter()
            .fold(1, |num, ans| num * ans / gcd(num, *ans))
            .to_string()
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

fn count_steps<F>(
    instructions: &Vec<Instruction>,
    start_node: &Node,
    nodes: &Vec<Node>, 
    end_condition: F
) -> isize
where F: Fn(&String) -> bool 
{
    let mut steps = 0;
    let mut node = start_node.clone();

    for instruction in instructions.iter().cycle() {
        let target = match instruction {
            Instruction::Left => &node.left,
            Instruction::Right => &node.right,
        };

        steps += 1;
        if end_condition(target) {
            break;
        }

        node = nodes.iter()
            .find(|n| n.id == target.to_owned())
            .unwrap()
            .clone();
    }

    steps
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
       a
    } else {
        gcd(b, a % b)
    }
}