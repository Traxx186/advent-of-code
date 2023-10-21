use std::collections::{HashSet, VecDeque};

use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day18 {}

impl Solution for Day18 {

    fn name(&self) -> String {
        "Boiling Boulders".to_owned()
    }

    fn part_1(&self) -> String {
        let cubes = parse_input(load("day_18"));
        let mut vissible_faces = 0;

        for cube in cubes.iter() {
            let covered_sides = cubes.iter()
                .filter(|other| &cube != other && cube.adjacent(other))
                .collect_vec()
                .len();

            vissible_faces += 6 - covered_sides;
        }

        vissible_faces.to_string()
    }

    fn part_2(&self) -> String {
        let cubes = parse_input(load("day_18"));

        let min_x = cubes.iter().copied().map(|cube| cube.x).min().unwrap();
        let max_x = cubes.iter().copied().map(|cube| cube.x).max().unwrap();
        let min_y = cubes.iter().copied().map(|cube| cube.y).min().unwrap();
        let max_y = cubes.iter().copied().map(|cube| cube.y).max().unwrap();
        let min_z = cubes.iter().copied().map(|cube| cube.z).min().unwrap();
        let max_z = cubes.iter().copied().map(|cube| cube.z).max().unwrap();

        let start = (min_x - 10, min_y - 10, min_z - 10);
        let exterior_area = count_exterior_area(
            start,
            &(
                (min_x - 10, max_x + 10),
                (min_y - 10, max_y + 10),
                (min_z - 10, max_z + 10),
            ),
            &cubes,
        );

        exterior_area.to_string()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Cube {
    x: isize,
    y: isize,
    z: isize
}

impl Cube {
    
    fn adjacent(&self, other: &Self) -> bool {
        self.adjacent_x(other) || self.adjacent_y(other) || self.adjacent_z(other)
    }

    fn adjacent_x(&self, other: &Self) -> bool {
        [-1, 1].into_iter().any(|d| {
            &Cube {
                x: self.x + d,
                y: self.y,
                z: self.z,
            } == other
        })
    }

    fn adjacent_y(&self, other: &Self) -> bool {
        [-1, 1].into_iter().any(|d| {
            &Cube {
                x: self.x,
                y: self.y + d,
                z: self.z,
            } == other
        })
    }

    fn adjacent_z(&self, other: &Self) -> bool {
        [-1, 1].into_iter().any(|d| {
            &Cube {
                x: self.x,
                y: self.y,
                z: self.z + d,
            } == other
        })
    }

}

impl From<(isize, isize, isize)> for Cube {

    fn from(values: (isize, isize, isize)) -> Self {
        Cube {
            x: values.0,
            y: values.1,
            z: values.2
        }
    }
}

fn parse_input(input: String) -> Vec<Cube> {
    input.lines()
        .map(|line| {
            line.split(',')
                .into_iter()
                .map(|split| split.parse::<isize>().unwrap())
                .collect_tuple::<(isize, isize, isize)>()
                .unwrap()
        })
        .map(Cube::from)
        .collect_vec()
}

type BBox = ((isize, isize), (isize, isize), (isize, isize));
fn count_exterior_area(start: (isize, isize, isize), bbox: &BBox, cubes: &Vec<Cube>) -> usize {
    let mut queue = VecDeque::new();
    let mut used = HashSet::new();
    
    queue.push_front(start);
    used.insert(start);

    let mut result = 0;
    let &((min_x, max_x), (min_y, max_y), (min_z, max_z)) = bbox;

    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();
        let cube = Cube { x, y, z };
        
        for other in cubes {
            if other.adjacent(&cube) {
                result += 1;
            }
        }

        for direction in [-1, 1] {
            let x_cube = Cube { x: x + direction, y, z };
            let y_cube = Cube { x, y: y + direction, z };
            let z_cube = Cube { x, y, z: z + direction };

            if min_x <= x + direction && max_x >= x + direction
                && !used.contains(&(x + direction, y, z))  && !cubes.contains(&x_cube) {
                queue.push_back((x + direction, y, z));
                used.insert((x + direction, y, z));
            }

            if min_y <= y + direction && max_y >= y + direction
                && !used.contains(&(x, y + direction, z))  && !cubes.contains(&y_cube) {
                queue.push_back((x, y + direction, z));
                used.insert((x, y + direction, z));
            }

            if min_z <= z + direction && max_z >= z + direction
                && !used.contains(&(x, y, z + direction))  && !cubes.contains(&z_cube) {
                queue.push_back((x, y, z + direction));
                used.insert((x, y, z + direction));
            }
        }
    }

    result
}