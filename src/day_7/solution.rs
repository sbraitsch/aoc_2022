use std::{collections::HashMap, time::Instant, fs::File, io::{BufReader, Lines}};
use super::utils::read_lines;


pub fn day7() {
    let now = Instant::now();
    let commands = read_lines("src/day_7/input.txt");

    let dir_structure = parse_file_structure(commands.unwrap());

    println!("Day 6 | Part 1: {:?}\nDay 6 | Part 2: {:?}", solution_1(&dir_structure), solution_2(&dir_structure));
    println!("Took: {:?}", now.elapsed());
}

fn solution_1(directories: &HashMap<Vec<String>, usize>) -> usize {
    directories.iter().filter(|(_, s)| s <= &&100000).map(|(_,s)| s).sum()
}

fn solution_2(directories: &HashMap<Vec<String>, usize>) -> usize {
    let unused_space = 70000000 - directories.get(&vec!["/".to_string()]).unwrap();
    let space_to_free = 30000000 - unused_space;
    directories.iter().filter(|(_, s)| s >= &&space_to_free).map(|(_,s)| *s).into_iter().min().unwrap()
}

fn parse_file_structure(commands: Lines<BufReader<File>>) -> HashMap<Vec<String>, usize> {
    let mut directory_sizes: HashMap<Vec<String>, usize> = HashMap::new();
    let mut current_dir: Vec<String> = Vec::new();
    let mut current_dir_size = 0;

    for command in commands {

        if let Ok(cmd) = command {
            let split = cmd.split(" ").collect::<Vec<&str>>();
            if split[1] == "cd" {
                update_past_dirs(&current_dir, &mut directory_sizes, current_dir_size);
                match split[2] {
                    ".." => {
                        current_dir.pop();
                    }
                    folder => {
                        current_dir.push(folder.to_string());
                    }
                }
                current_dir_size = 0;
            } else if let Ok(size) = split[0].parse::<usize>() {
                current_dir_size += size;
            }
        }
    }

    update_past_dirs(&current_dir, &mut directory_sizes, current_dir_size);

    directory_sizes
}

fn update_past_dirs(current_dir: &Vec<String>, directory_sizes: &mut HashMap<Vec<String>, usize>, current_dir_size: usize) {
    let mut parent_directories: Vec<String> = Vec::new();
    for partial in current_dir.iter() {
        parent_directories.push(partial.to_string());
        if !directory_sizes.contains_key(&parent_directories) {
            directory_sizes.insert(parent_directories.clone(), 0);
        }
        *directory_sizes.get_mut(&parent_directories).unwrap() += current_dir_size;
    }
}