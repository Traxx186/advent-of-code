use std::cmp::Ordering;

use aoc_common::{load, Solution};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;

pub struct Day13 {}

impl Solution for Day13 {
    fn name(&self) -> String {
        "Distress Signal".to_owned()
    }

    fn part_1(&self) -> String {
        let input = parse_input(&load("day_13"));

        let out: usize = input
            .into_iter()
            .enumerate()
            .filter(|(_, (a, b))| a < b)
            .map(|(i, _)| i + 1)
            .sum();

        out.to_string()
    }

    fn part_2(&self) -> String {
        let distress_1 = Packet::new_list(vec![Packet::new_list(vec![Packet::new_int(2)])]);
        let distress_2 = Packet::new_list(vec![Packet::new_list(vec![Packet::new_int(6)])]);

        let mut packets = parse_input(&load("day_13"))
            .into_iter()
            .flat_map(|(group_1, group_2)| [group_1, group_2])
            .chain([distress_1.clone(), distress_2.clone()])
            .collect_vec();

        packets.sort();

        let position_1 = packets
            .iter()
            .position(|p| p == &distress_1)
            .map(|i| i + 1)
            .unwrap();
        let position_2 = packets
            .iter()
            .position(|p| p == &distress_2)
            .map(|i| i + 1)
            .unwrap();

        (position_1 * position_2).to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn new_int(int: usize) -> Self {
        Self::Int(int)
    }

    fn new_list(list: Vec<Self>) -> Self {
        Self::List(list)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(ref l), Packet::Int(ref r)) => l.partial_cmp(r),
            (Packet::List(packets), Packet::List(other_packets)) => {
                let order = packets
                    .iter()
                    .zip(other_packets)
                    .fold(None, |order, (a, b)| {
                        order.or(match a.cmp(b) {
                            Ordering::Less => Some(Ordering::Less),
                            Ordering::Equal => None,
                            Ordering::Greater => Some(Ordering::Greater),
                        })
                    });

                order.or_else(|| packets.len().partial_cmp(&other_packets.len()))
            }
            (Packet::List(_), Packet::Int(_)) => {
                self.partial_cmp(&Packet::new_list(vec![other.clone()]))
            }
            (Packet::Int(_), Packet::List(_)) => {
                Packet::new_list(vec![self.clone()]).partial_cmp(other)
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    let (_, packets) = separated_list0(pair(line_ending, line_ending), parse_pair_line)(input)
        .expect("Failed to parse input");

    packets
}

fn parse_pair_line(input: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(
        alt((parse_int_packet, parse_list_packet)),
        line_ending,
        alt((parse_int_packet, parse_list_packet)),
    )(input)
}

fn parse_int_packet(input: &str) -> IResult<&str, Packet> {
    map(map_res(digit1, str::parse), Packet::new_int)(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Packet>> {
    delimited(tag("["), separated_list0(tag(","), packet), tag("]"))(input)
}

fn parse_list_packet(input: &str) -> IResult<&str, Packet> {
    map(parse_list, Packet::new_list)(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((parse_int_packet, parse_list_packet))(input)
}
