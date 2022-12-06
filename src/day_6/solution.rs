use std::collections::HashSet;

use super::utils::read_string;

pub fn day6() {
    let sequence = read_string("src/day_6/input.txt").chars().collect::<Vec<char>>();

    let sol_1 = solution_1(&sequence);
    let sol_2 = solution_2(&sequence);

    println!("Day 6 | Part 1: {:?}\nDay 6 | Part 2: {:?}", sol_1, sol_2)
}

fn solution_1(sequence: &Vec<char>) -> usize {
    index_of_unique_window(sequence, 4)
}

fn solution_2(sequence: &Vec<char>) -> usize {
    index_of_unique_window(sequence, 14)
}

fn index_of_unique_window(sequence: &Vec<char>, window_size: usize) -> usize {
    for (i, window) in sequence.windows(window_size).enumerate() {
        let mut uniques = HashSet::new();
        window.iter().for_each(|e| { uniques.insert(e.clone()); });
        if uniques.len() == window_size {
            return i + window_size
        }
    }
    0
}