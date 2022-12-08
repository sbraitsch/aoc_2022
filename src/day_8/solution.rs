use std::{time::Instant, collections::HashMap};
use crate::utils::input_lines;

#[derive(Clone, Debug)]
struct Tree {
    height: usize,
    visible: bool,
    scenic_score: usize
}

type Forest = HashMap<(usize, usize), Tree>;

pub fn day8() {
    let now = Instant::now();
    let mut forest: Forest = HashMap::new();

    for (y, row) in input_lines(8).iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            forest.insert((x, y), Tree { height:c.to_digit(10).unwrap() as usize, visible: true, scenic_score: 1});
        }
    }

    map_visibility(&mut forest);

    println!("Day 8 | Part 1: {:?}\nDay 8 | Part 2: {:?}", solution_1(&forest), solution_2(&forest));
    println!("Took: {:?}", now.elapsed());
}

fn solution_1(forest: &Forest) -> usize {
    forest.iter().fold(0, |acc, (_, tree)| acc + tree.visible as usize)
}

fn solution_2(forest: &Forest) -> usize {
    forest.iter().map(|(_, tree)| tree.scenic_score).max().unwrap()
}

fn map_visibility(forest: &mut Forest) {
    let f = forest.clone();

    for ((x, y), tree) in forest.iter_mut() {
        if !(x == &0 || y == &0 || x == &98 || y == &98) {
            let mut blocked = 0;
            let mut cur_vd = 0;
            let mut scenic_score = 1;
            
            //west
            for (vd, i) in (0..*x).rev().enumerate() {
                cur_vd = vd+1;
                if f[&(i, *y)].height >= tree.height {
                    blocked += 1;
                    break;
                }
            }
            scenic_score *= cur_vd;

            //east
            for (vd, i) in (*x+1..99).enumerate() {
                cur_vd = vd+1;
                if f[&(i, *y)].height >= tree.height {
                    blocked += 1;
                    break;
                }
            }
            scenic_score *= cur_vd;

            //north
            for (vd, i) in (0..*y).rev().enumerate() {
                cur_vd = vd+1;
                if f[&(*x, i)].height >= tree.height {
                    blocked += 1;
                    break;
                }
            }
            scenic_score *= cur_vd;

            //south
            for (vd, i) in (*y+1..99).enumerate() {
                cur_vd = vd+1;
                if f[&(*x, i)].height >= tree.height {
                    blocked += 1;
                    break;
                }
            }
            scenic_score *= cur_vd;

            update_map(scenic_score, blocked, tree);
        }
    }
}

fn update_map(scenic_score: usize, blocked: u8, tree: &mut Tree) {
    if blocked == 4 { tree.visible = false};
    tree.scenic_score *= scenic_score;
}