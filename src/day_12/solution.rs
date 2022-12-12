use std::{time::Instant};

use crate::utils::input_lines;

#[derive(Debug, Clone)]
struct Node {
    idx: usize,
    cost: usize,
    g_cost: usize,
    h_cost: usize,
    prev: Option<usize>
}

pub fn day12() {
    let now = Instant::now();
    let mut map: Vec<Node> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut target: (usize, usize) = (0, 0);
    let input = input_lines(12);
    let dim_x = input.first().unwrap().len();
    let dim_y = input.len();
    let mut all_basins: Vec<(usize, usize)> = Vec::new();

    for (y, row) in input.iter().enumerate() {
        for (x, height) in row.chars().enumerate() {
            match height {
                'S' => { 
                    start = (x, y);
                    map.push(Node { idx: (y * dim_x) + x, cost: 1, g_cost: 0, h_cost: 0, prev: None});
                },
                'E' => { 
                    target = (x, y);
                    map.push(Node { idx: (y * dim_x) + x, cost: 26, g_cost: 0, h_cost: 0, prev: None});
                },
                c => {
                    let height = c as usize - 96;
                    if height == 1 { all_basins.push((x, y)) }
                    map.push(Node { idx: (y * dim_x) + x, cost: height, g_cost: 0, h_cost: 0, prev: None});
                }
            }
        }
    }

    for node in map.iter_mut() {
        node.h_cost = calc_h_cost((node.idx % dim_x, node.idx / dim_x), target)
    }

    println!("Day 12 | Part 1: {:?}\nDay 12 | Part 2: {:?}", solution_1(&mut map.clone(), start, target, dim_x, dim_y), solution_2(&mut map, &mut all_basins, target, dim_x, dim_y));
    println!("Took: {:?}", now.elapsed());
}

fn solution_1(map: &mut Vec<Node>, start: (usize, usize), target: (usize, usize), dim_x: usize, dim_y: usize) -> usize {
    astar(map, start, target, dim_x, dim_y)
}

fn solution_2(map: &mut Vec<Node>, basins: &mut Vec<(usize, usize)>, target: (usize, usize), dim_x: usize, dim_y: usize) -> usize {
    let mut current_min = 1000;
    for basin in basins {
        let path_len = astar(&mut map.clone(), *basin, target, dim_x, dim_y);
        if path_len != 0 {
            println!("Found a path with length <{path_len}> starting from <{:?}>", basin);
            if path_len < current_min {
                current_min = path_len;
            }
        }
    }
    current_min
}

fn get_adjacent<'a>(idx: usize, dim_x: usize, dim_y: usize) -> Vec<usize> {
    let x = (idx % dim_x) as i16;
    let y = (idx / dim_x) as i16;
    let adj = vec![
        (x, y - 1),
        (x, y + 1),
        (x - 1, y),
        (x + 1, y)
    ];

    adj.iter()
        .filter(|(x, y)| x >= &0 && x <= &((dim_x - 1) as i16) && y >= &0 && y <= &((dim_y - 1) as i16))
        .map(|(x, y)| get_idx(*x as usize, *y as usize, dim_x))
        .collect()
}

fn calc_h_cost(start: (usize, usize), end: (usize, usize)) -> usize {
    end.0.abs_diff(start.0) + end.1.abs_diff(start.1)
}

fn get_idx(x: usize, y: usize, dim_x: usize) -> usize {
    (y * dim_x) + x
}

fn min_by_fcost(maze: &Vec<Node>, open: &Vec<usize>) -> (usize, usize) {
    let mut min_idx = (0, 0);
    let mut current_min = 1000;
    for (i, idx) in open.iter().enumerate() {
        let check = &maze[*idx];
        let f_cost = check.g_cost + check.h_cost;
        if f_cost < current_min {
            current_min = f_cost;
            min_idx = (i, *idx);
        }
    }
    min_idx
}

fn astar(maze: &mut Vec<Node>, (start_x, start_y): (usize, usize), (target_x, target_y): (usize, usize), dim_x: usize, dim_y: usize)-> usize {
    let mut open_list: Vec<usize> = Vec::new();
    let mut closed_list: Vec<usize> = Vec::new();

    let target = get_idx(target_x, target_y, dim_x);
    open_list.push(get_idx(start_x, start_y, dim_x));

    while !open_list.is_empty() {
        let (open_idx, mut current_idx) = min_by_fcost(&maze, &open_list);
        open_list.remove(open_idx);
        if current_idx == target {
            let mut path_len = 0;
            while let Some(idx) = maze[current_idx].prev {
                maze[idx].cost = 0;
                current_idx = idx;
                path_len += 1;
            };
            return path_len;
        } else {
            closed_list.push(current_idx);
            for adj in get_adjacent(current_idx, dim_x, dim_y) {
                if !closed_list.contains(&adj) {
                    if maze[adj].cost as i8 - maze[current_idx].cost as i8 > 1 { continue; };
                    let cost_through_current = maze[current_idx].g_cost + 1;
                    if !open_list.contains(&adj) {
                        maze[adj].g_cost = cost_through_current;
                        maze[adj].prev = Some(current_idx);
                        open_list.push(adj);
                    } else{
                        if cost_through_current < maze[adj].g_cost {
                            maze[adj].g_cost = cost_through_current;
                            maze[adj].prev = Some(current_idx)
                        }
                    }
                }
            }
        }
    }
    0
}