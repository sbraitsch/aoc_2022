use std::{time::Instant, collections::BTreeSet};

use crate::utils::input_lines;

#[derive(Debug, Clone)]
struct Node {
    idx: usize,
    elevation: usize,
    cost: usize
}

pub fn day12() {
    let now = Instant::now();
    let mut map: Vec<Node> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut target: (usize, usize) = (0, 0);
    let input = input_lines(12);
    let dim_x = input.first().unwrap().len();
    let dim_y = input.len();

    for (y, row) in input.iter().enumerate() {
        for (x, height) in row.chars().enumerate() {
            match height {
                'S' => { 
                    start = (x, y);
                    map.push(Node { idx: (y * dim_x) + x, cost: 0, elevation: 1});
                },
                'E' => { 
                    target = (x, y);
                    map.push(Node { idx: (y * dim_x) + x, cost: 0, elevation: 26});
                },
                c => {
                    let height = c as usize - 96;
                    map.push(Node { idx: (y * dim_x) + x, cost: 0, elevation: height});
                }
            }
        }
    }

    println!("Day 12 | Part 1: {:?}\nDay 12 | Part 2: {:?}", solution_1(&mut map.clone(), start, target, dim_x, dim_y), solution_2(&mut map.clone(), target, dim_x, dim_y));
    println!("Benchmark: {:?}", now.elapsed());
}

fn solution_1(map: &mut Vec<Node>, start: (usize, usize), target: (usize, usize), dim_x: usize, dim_y: usize) -> usize {
    shortest_path(map, start, target, dim_x, dim_y, false)
}

fn solution_2(map: &mut Vec<Node>, target: (usize, usize), dim_x: usize, dim_y: usize) -> usize {
    shortest_path(map, target, (0, 0), dim_x, dim_y, true)
}

fn get_adjacent<'a>(idx: usize, dim_x: usize, dim_y: usize) -> Vec<usize> {
    let x = idx % dim_x;
    let y = idx / dim_x;

    let mut adj = Vec::new();
    if x > 0 { adj.push(idx - 1); }
    if x < dim_x - 1 { adj.push(idx + 1); }
    if y > 0 { adj.push(idx - dim_x); }
    if y < dim_y - 1 { adj.push(idx + dim_x); }

    adj
}

fn get_idx(x: usize, y: usize, dim_x: usize) -> usize {
    (y * dim_x) + x
}

fn shortest_path(maze: &mut Vec<Node>, (start_x, start_y): (usize, usize), (target_x, target_y): (usize, usize), dim_x: usize, dim_y: usize, p2: bool)-> usize {
    let mut open_list = BTreeSet::new();
    let mut closed_list: Vec<usize> = Vec::new();

    let target = get_idx(target_x, target_y, dim_x);
    open_list.insert((0, get_idx(start_x, start_y, dim_x)));

    while !open_list.is_empty() {
        let (cur_cost, current_idx) = open_list.pop_first().unwrap();
        let current_node = maze[current_idx].clone();
        if (p2 && current_node.elevation == 1) || (!p2 && current_idx == target) {
            return current_node.cost;
        } else {
            closed_list.push(current_idx);
            for adj in get_adjacent(current_idx, dim_x, dim_y) {
                let adj_node = &mut maze[adj];
                if !closed_list.contains(&adj) {
                    if adj_node.elevation as i8 - current_node.elevation as i8 > 1 && !p2 { 
                        continue; 
                    } else if current_node.elevation as i8 - adj_node.elevation as i8 > 1 && p2 {
                        continue;
                    };
                    let cost_through_current = cur_cost + 1;
                    if open_list.iter().find(|(_, i)| i == &adj).is_none() {
                        adj_node.cost = cost_through_current;
                        open_list.insert((cost_through_current, adj));
                    } else{
                        if cost_through_current < adj_node.cost {
                            adj_node.cost = cost_through_current;
                        }
                    }
                }
            }
        }
    }
    0
}
