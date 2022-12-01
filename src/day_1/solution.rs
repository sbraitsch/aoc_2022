use std::collections::BinaryHeap;

#[path = "../util/file_reader.rs"] mod fr;

pub fn day1() {
    let mut sum_vec: Vec<usize> = Vec::new();

    if let Ok(lines) = fr::read_lines("src/day_1/input.txt") {
        let mut temp_sum = 0;
        for line in lines {
            if let Ok(calories) = line {
               if calories == ""  {
                sum_vec.push(temp_sum);
                temp_sum = 0
               } else {
                temp_sum += calories.parse::<usize>().unwrap();
               }
            }
        }
    }
    let sol_1 = solution_1(&sum_vec);
    let sol_2 = solution_2(sum_vec.iter().copied().collect::<BinaryHeap<usize>>());

    println!("Day 1 | Part 1: {:?}\nDay 1 | Part 2: {:?}", sol_1, sol_2);
}

pub fn solution_1(calories: &Vec<usize>) -> usize {
    *calories.iter().max().unwrap()
}

fn solution_2(mut calories: BinaryHeap<usize>) -> usize {
    let mut top_three_sum = 0;
    for _ in 0..3 {
        if let Some(top) = calories.pop() {
            top_three_sum += top;
        }
    }
    top_three_sum
}