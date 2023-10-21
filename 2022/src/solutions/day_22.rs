use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use aoc_common::{
    load,
    point::{Point2D, Point3D},
    Solution,
};
use num::integer::Roots;
use regex::Regex;


const TILE_OPEN: char = '.';
const TILE_WALL: char = '#';

const DIRECTIONS: [Point2D<isize>; 4] = [
    // R, D, L, U
    Point2D { x: 1, y: 0 },
    Point2D { x: 0, y: 1 },
    Point2D { x: -1, y: 0 },
    Point2D { x: 0, y: -1 },
];

const DIRECTIONS_3D: [Point3D<isize>; 4] = [
    // R, D, L, U
    Point3D { x: 1, y: 0, z: 0 },
    Point3D { x: 0, y: -1, z: 0 },
    Point3D { x: -1, y: 0, z: 0 },
    Point3D { x: 0, y: 1, z: 0 },
];

pub struct Day22 {}

impl Solution for Day22 {
    fn name(&self) -> String {
        "Monkey Map".to_owned()
    }

    fn part_1(&self) -> String {
        let map = parse_input(load("day_22"), WrapMode::Flat);
        let (position, facing) = walk(&map);
        let super_secret_pasword = (position.y + 1) * 1000 + (position.x + 1) * 4 + facing as isize;

        super_secret_pasword.to_string()
    }

    fn part_2(&self) -> String {
        let map = parse_input(load("day_22"), WrapMode::Cube);
        let (position, facing) = walk_cube(&map);
        let super_secret_pasword = (position.y + 1) * 1000 + (position.x + 1) * 4 + facing as isize - 1;

        super_secret_pasword.to_string()
    }
}

fn walk(map: &Map) -> (Point2D<isize>, usize) {
    let mut position = map.start.clone();
    let mut facing = 0;

    for (facing_next, distance) in &map.path {
        facing = (facing + *facing_next) % 4;

        for _ in 0..*distance {
            let (next_position, change) = get_next_position(position, facing, map);

            match map.tiles.get(&next_position) {
                Some(&c) if c == TILE_WALL => break,
                Some(_) => {
                    position = next_position;
                    facing = (facing as isize + change + 4) as usize % 4;
                }
                None => Err("Oops found void").unwrap(),
            }
        }
    }

    (position, facing)
}

fn walk_cube(map: &Map) -> (Point2D<isize>, usize) {
    let mut cube = map.cube.clone();
    let mut position_3d = cube.get_position_3d(&map.start.clone()).unwrap();
    let mut facing_3d = 0;
    let mut visited = vec![];

    for (facing_next, distance) in map.path.iter() {
        facing_3d = (facing_3d + *facing_next) % 4;
        let direction_3d = DIRECTIONS_3D[facing_3d];

        for _ in 0..*distance {
            let cube_origin = cube.clone();
            let mut next_position_3d = position_3d + direction_3d;

            match next_position_3d {
                p if p.x < 0 => {
                    cube.rotate(rad(90), 0.0, 0.0);
                    next_position_3d.x = map.side_length - 1;
                }
                p if p.x >= map.side_length => {
                    cube.rotate(rad(-90), 0.0, 0.0);
                    next_position_3d.x = 0;
                }
                p if p.y < 0 => {
                    cube.rotate(0.0, rad(-90), 0.0);
                    next_position_3d.y = map.side_length - 1;
                }
                p if p.y >= map.side_length => {
                    cube.rotate(0.0, rad(90), 0.0);
                    next_position_3d.y = 0;
                }
                _ => (),
            }

            let position_2d = *cube.tiles.get(&next_position_3d).unwrap();
            if *map.tiles.get(&position_2d).ok_or("invalid 3d map").unwrap() == TILE_WALL {
                cube = cube_origin;
                break;
            }

            visited.push(position_2d);
            position_3d = next_position_3d;
        }
    }

    let position_2d = visited[visited.len() - 1];
    let facing = DIRECTIONS
        .iter()
        .position(|&p| p == (position_2d - visited[visited.len() - 2]))
        .unwrap();

    (position_2d, facing)
}

fn get_next_position(start: Point2D<isize>, facing: usize, map: &Map) -> (Point2D<isize>, isize) {
    let position = start + DIRECTIONS[facing];
    let (next_position, facing_change) = match facing {
        0 | 2 => *map.portals_h.get(&position).unwrap_or(&(position, 0)),
        1 | 3 => *map.portals_v.get(&position).unwrap_or(&(position, 0)),
        _ => Err("invalid facing").unwrap(),
    };

    (next_position, facing_change)
}

enum WrapMode {
    Flat,
    Cube,
}

#[derive(Debug, Default, Clone)]
struct Cube {
    side_length: isize,
    tiles: HashMap<Point3D<isize>, Point2D<isize>>,
}

impl Cube {
    fn get_position_3d(&self, point: &Point2D<isize>) -> Option<Point3D<isize>> {
        self.tiles
            .iter()
            .find(|(_, &p2)| p2 == *point)
            .map(|x| *x.0)
    }

    fn rotate(&mut self, pitch: f32, roll: f32, yaw: f32) {
        let cosa = yaw.cos();
        let sina = yaw.sin();
        let cosb = pitch.cos();
        let sinb = pitch.sin();
        let cosc = roll.cos();
        let sinc = roll.sin();
        let axx = cosa * cosb;
        let axy = cosa * sinb * sinc - sina * cosc;
        let axz = cosa * sinb * cosc + sina * sinc;
        let ayx = sina * cosb;
        let ayy = sina * sinb * sinc + cosa * cosc;
        let ayz = sina * sinb * cosc - cosa * sinc;
        let azx = -sinb;
        let azy = cosb * sinc;
        let azz = cosb * cosc;

        let t = Point3D {
            x: (self.side_length as f32 - 1.0) / -2.0,
            y: (self.side_length as f32 - 1.0) / -2.0,
            z: (self.side_length as f32 - 1.0) / 2.0,
        };

        // translate
        let tiles_f = self.tiles.drain().map(|(p, c)| {
            (
                Point3D::<f32> {
                    x: p.x as f32 + t.x,
                    y: p.y as f32 + t.y,
                    z: p.z as f32 + t.z,
                },
                c,
            )
        });

        // rotate
        let tiles_f = tiles_f.map(|(p, c)| {
            let p2 = Point3D {
                x: axx * p.x + axy * p.y + axz * p.z,
                y: ayx * p.x + ayy * p.y + ayz * p.z,
                z: azx * p.x + azy * p.y + azz * p.z,
            };
            (p2, c)
        });
        
        // translate back
        self.tiles = HashMap::from_iter(tiles_f.map(|(p, c)| {
            (
                Point3D::<isize> {
                    x: (p.x - t.x).round() as isize,
                    y: (p.y - t.y).round() as isize,
                    z: (p.z - t.z).round() as isize,
                },
                c,
            )
        }));
    }
}

#[derive(Debug, Default)]
struct Map {
    start: Point2D<isize>,
    cube: Cube,
    tiles: HashMap<Point2D<isize>, char>,
    width: isize,
    height: isize,
    side_length: isize,
    path: Vec<(usize, isize)>,
    portals_h: HashMap<Point2D<isize>, (Point2D<isize>, isize)>,
    portals_v: HashMap<Point2D<isize>, (Point2D<isize>, isize)>,
}

fn parse_input(input: String, mode: WrapMode) -> Map {
    let (input_map, input_path) = input.split_once("\n\n").ok_or("Invalid input").unwrap();

    let mut tiles = HashMap::new();
    let mut ranges_x: HashMap<isize, RangeInclusive<isize>> = HashMap::new();
    let mut ranges_y: HashMap<isize, RangeInclusive<isize>> = HashMap::new();
    let mut start_x = None;
    for (y, line) in input_map.lines().enumerate() {
        let y = y as isize;

        for (x, character) in line.chars().enumerate() {
            let x = x as isize;
            if character != TILE_OPEN && character != TILE_WALL {
                continue;
            }

            if y == 0 && start_x.is_none() && character == TILE_OPEN {
                start_x = Some(x);
            }

            tiles.insert(Point2D { x, y }, character);
            ranges_x
                .entry(y)
                .and_modify(|r| *r = *r.start().min(&x)..=*r.end().max(&x))
                .or_insert(x..=x);

            ranges_y
                .entry(x)
                .and_modify(|r| *r = *r.start().min(&y)..=*r.end().max(&y))
                .or_insert(y..=y);
        }
    }

    let width = ranges_x
        .iter()
        .map(|(_, r)| *r.end())
        .max()
        .ok_or("Map is empty")
        .unwrap()
        + 1;
    let height = ranges_y
        .iter()
        .map(|(_, r)| *r.end())
        .max()
        .ok_or("Map is empty")
        .unwrap()
        + 1;
    let side_length = (tiles.len() / 6).sqrt() as isize;

    let mut facing = 0;
    let mut path = vec![];
    let regex = Regex::new(r"(-?\d+)([LR])?").unwrap();
    for captures in regex.captures_iter(input_path) {
        let distance = captures
            .get(1)
            .and_then(|x| x.as_str().parse::<isize>().ok())
            .ok_or("Invalid path")
            .unwrap();

        path.push((facing, distance));

        let turn = captures.get(2).and_then(|x| x.as_str().chars().next());
        match turn {
            Some(character) if character == 'R' => facing = 1,
            Some(character) if character == 'L' => facing = 3,
            Some(_) => Err("invalid path").unwrap(),
            None => (),
        }
    }

    let mut map = Map {
        start: Point2D {
            x: start_x.ok_or("Start position not found").unwrap(),
            y: 0,
        },
        tiles,
        path,
        width,
        height,
        side_length,
        ..Default::default()
    };

    match mode {
        WrapMode::Flat => add_flat_portals(&mut map, &ranges_x, &ranges_y),
        WrapMode::Cube => add_cube(&mut map),
    }

    map
}

fn add_flat_portals(
    map: &mut Map,
    ranges_x: &HashMap<isize, RangeInclusive<isize>>,
    ranges_y: &HashMap<isize, RangeInclusive<isize>>,
) {
    let portals_h = &mut map.portals_h;
    for (y, range) in ranges_x {
        portals_h.insert(
            Point2D {
                x: range.start() - 1,
                y: *y,
            },
            (
                Point2D {
                    x: *range.end(),
                    y: *y,
                },
                0,
            ),
        );
        portals_h.insert(
            Point2D {
                x: range.end() + 1,
                y: *y,
            },
            (
                Point2D {
                    x: *range.start(),
                    y: *y,
                },
                0,
            ),
        );
    }

    let portals_v = &mut map.portals_v;
    for (x, range) in ranges_y {
        portals_v.insert(
            Point2D {
                x: *x,
                y: range.start() - 1,
            },
            (
                Point2D {
                    x: *x,
                    y: *range.end(),
                },
                0,
            ),
        );
        portals_v.insert(
            Point2D {
                x: *x,
                y: range.end() + 1,
            },
            (
                Point2D {
                    x: *x,
                    y: *range.start(),
                },
                0,
            ),
        );
    }
}

fn add_cube(map: &mut Map) {
    let mut sides = vec![];
    for y in 0..(map.height / map.side_length) {
        for x in 0..(map.width / map.side_length) {
            let tile_position = Point2D {
                x: x * map.side_length,
                y: y * map.side_length,
            };
            if map.tiles.contains_key(&tile_position) {
                sides.push(Point2D { x, y });
            }
        }
    }

    let start = sides[0].clone();
    let side_map: HashSet<Point2D<isize>> = HashSet::from_iter(sides);
    let mut cube = Cube {
        side_length: map.side_length,
        ..Default::default()
    };

    fill_cube(&start, &side_map, &mut HashSet::new(), &mut cube, &map);
    map.cube = cube;
}

fn fill_cube(
    side: &Point2D<isize>,
    sides: &HashSet<Point2D<isize>>,
    visited: &mut HashSet<Point2D<isize>>,
    cube: &mut Cube,
    map: &Map,
) {
    for y in side.y * cube.side_length..side.y * cube.side_length + cube.side_length {
        for x in side.x * cube.side_length..side.x * cube.side_length + cube.side_length {
            let from = Point2D { x, y };
            let to = Point3D {
                x: x - side.x * cube.side_length,
                y: map.side_length - 1 - (y - side.y * cube.side_length), // flip y
                z: 1,
            };

            cube.tiles.insert(to, from);
        }
    }

    for (dir_idx, dir) in DIRECTIONS.iter().enumerate() {
        let next_side = *side + *dir;

        if visited.insert(next_side) && sides.contains(&next_side) {
            match dir_idx {
                0 | 2 => cube.rotate(rad((dir_idx as isize - 1) * 90), 0.0, 0.0),
                _ => cube.rotate(0.0, rad((dir_idx as isize - 2) * 90), 0.0),
            }
            fill_cube(&next_side, sides, visited, cube, map);
            match dir_idx {
                0 | 2 => cube.rotate(rad((dir_idx as isize - 1) * -90), 0.0, 0.0),
                _ => cube.rotate(0.0, rad((dir_idx as isize - 2) * -90), 0.0),
            }
        }
    }
}

fn rad(deg: isize) -> f32 {
    (deg as f32).to_radians()
}
