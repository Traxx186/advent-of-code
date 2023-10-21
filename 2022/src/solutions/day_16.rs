use std::{
    cmp::max,
    collections::{HashMap, VecDeque} 
};

use aoc_common::{Solution, load};
use itertools::Itertools;
use regex::Regex;


pub struct Day16 {}

const MAX_TIME_PART_1: usize = 30;
const MAX_TIME_PART_2: usize = 26;

impl Solution for Day16 {

    fn name(&self) -> String {
        "Proboscidea Volcanium".to_owned()
    }

    fn part_1(&self) -> String {
        let (all_nodes, start) = parse_input(load("day_16"));
        let graph = get_weighted_graph(&all_nodes, start);
        let solver = Solver::new(graph, MAX_TIME_PART_1, 1);
        let (_solver, score) = solver.search(MAX_TIME_PART_1);

        score.to_string()
    }

    fn part_2(&self) -> String {
        let (all_nodes, start) = parse_input(load("day_16"));
        let graph = get_weighted_graph(&all_nodes, start);
        let solver = Solver::new(graph, MAX_TIME_PART_2, 2);
        let (_solver, score) = solver.search(MAX_TIME_PART_2);

        score.to_string()
    }
}

#[derive(Debug, Clone)]
struct Node {
    rate: usize,
    adjacent: Vec<usize>
}

#[derive(Debug)]
struct Solver {
    graph: WeightedGraph,
    steps: Vec<Vec<usize>>,
    time: Vec<usize>,
    position: Vec<usize>,
}

impl Solver {
    fn new(graph: WeightedGraph, max_time: usize, players: usize) -> Self {
        Self {
            graph,
            steps: vec![vec![0]; players],
            time: vec![max_time; players],
            position: vec![0; players],
        }
    }

    fn score(&self, max_time: usize) -> usize {
        let mut score = 0;

        for steps in self.steps.iter() {
            let mut time = max_time;

            for i in 1..steps.len() {
                time -= self.graph.weights[steps[i - 1]][steps[i]];
                score += self.graph.rates[steps[i]] * time;    
            }
        }

        score
    }

    fn advance(&mut self, next: usize, player: usize) {
        self.time[player] -= self.graph.weights[*self.steps[player].last().unwrap()][next];
        self.steps[player].push(next);
        self.position[player] = next;
    }

    fn backtrack(&mut self, player: usize) {
        let last = self.steps[player].pop().unwrap();
        let prev = *self.steps[player].last().unwrap();
        
        self.time[player] += self.graph.weights[prev][last];
        self.position[player] = prev;
    }

    fn options(&self, player: usize, time_left: usize) -> Vec<usize> {
        (1..self.graph.len()).into_iter()
            .filter(|i| {
                for step in self.steps.iter().flatten() {
                    if i == step {
                        return false;
                    }
                }

                self.graph.weights[self.position[player]][*i] <= time_left
            })
            .collect_vec()
    }

    fn search(mut self, max_time: usize) -> (Self, usize) {  
        let mut max_score = 0;      
        let options0 = self.options(0, self.time[0]);
        if options0.is_empty() {
            if self.time.len() > 1 {
                let options1 = self.options(1, self.time[1]);
                return self.search_moves(&options1, 1, max_time);    
            }

            let score = self.score(max_time);
            return (self, score);
        }


        for _ in 0..self.time.len() {
            for option0 in &options0 {
                self.advance(*option0, 0);
                let score;

                if self.time.len() > 1 {
                    let option1 = self.options(1, self.time[1]);
                    (self, score) = self.search_moves(&option1, 1, max_time);    
                } else {
                    (self, score) = self.search(max_time);
                }
    
                max_score = max(max_score, score);
                self.backtrack(0);
            }
        }
        

        (self, max_score)
    }

    fn search_moves(mut self, moves: &Vec<usize>, player: usize, max_time: usize) -> (Self, usize) {
        if moves.is_empty() {
            let score = self.score(max_time);
            return (self, score);
        }

        let mut max_score = 0;
        for &m in moves {
            self.advance(m, player);

            let score;
            (self, score) = self.search(max_time);

            max_score = max(max_score, score);
            self.backtrack(player);
        }

        (self, max_score)
    }
}

#[derive(Debug)]
struct WeightedGraph {
    rates: Vec<usize>,
    weights: Vec<Vec<usize>>,
}

impl WeightedGraph {
    fn new(rates: &Vec<usize>) -> Self {
        Self { 
            rates: rates.clone(),
            weights: vec![vec![0; rates.len()]; rates.len()],
        }
    }

    fn add_edge(&mut self, v: usize, w: usize, weight: usize) {
        self.weights[v][w] = weight;
        self.weights[w][v] = weight;
    }

    fn len(&self) -> usize {
        self.rates.len()
    }
}

fn get_weighted_graph(nodes: &Vec<Node>, start: usize) -> WeightedGraph {
    let mut rates = vec![nodes[start].rate];
    let mut valuable = vec![start];
    
    let mut to_value_id: Vec<Option<usize>> = vec![None; nodes.len()];
    to_value_id[start] = Some(0);

    for (i, node) in nodes.iter().enumerate() {
        if node.rate > 0 && i != start {
           to_value_id[i] = Some(rates.len());
            rates.push(node.rate);
            valuable.push(i);
        }
    }

    let mut graph = WeightedGraph::new(&rates);
    for value in valuable {
        let mut distance_to_value = vec![0; nodes.len()];
        let mut visited = vec![false; nodes.len()];
        let mut queue: VecDeque<usize> = VecDeque::new();
        
        queue.push_back(value);
        visited[value] = true;

        while !queue.is_empty() {
            let item = queue.pop_front().unwrap();
            let distance = distance_to_value[item];

            for &node in nodes[item].adjacent.iter() {
                if visited[node]{
                    continue;
                }

                visited[node] = true;
                distance_to_value[node] = distance + 1;
                
                if nodes[node].rate > 0 {
                    graph.add_edge(to_value_id[value].unwrap(), to_value_id[node].unwrap(), distance + 2);
                }

                queue.push_back(node);
            }
        }
    }

    graph
}

fn parse_input(input: String) -> (Vec<Node>, usize) {
    let regex = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ").unwrap();
    let mut nodes = Vec::new();
    let mut code_to_id: HashMap<&str, usize> = HashMap::new();
    let mut adjacent_code: Vec<Vec<&str>> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let capture = regex.captures(line).unwrap();        
        let name = capture.get(1).unwrap().as_str();
        code_to_id.insert(name, i);

        let rate = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let match_size = capture.get(0).unwrap().end();
        let adjacent = line[match_size..].split(", ").collect_vec();

        adjacent_code.push(adjacent);
        nodes.push(Node {
            rate,
            adjacent: Vec::new(),
        });
    }

    for (i, ajd_code) in adjacent_code.iter().enumerate() {
        let adjacent: Vec<usize> = ajd_code.into_iter()
            .map(|code| *code_to_id.get(code.trim()).unwrap())
            .collect_vec();

        nodes[i].adjacent = adjacent;
    }

    (nodes, code_to_id["AA"])
}