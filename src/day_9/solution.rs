use std::{time::Instant, collections::HashSet};
use crate::utils::input_lines;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn should_move(self: &Knot, trailing: &Knot) -> bool {
        self.x.abs_diff(trailing.x) > 1 || self.y.abs_diff(trailing.y) > 1
    }

    fn adjust_following(self: &Knot, next_knot: &mut Knot) {
        if self.should_move(next_knot) {
            let mut x_offset = 0;
            if self.x > next_knot.x {
                x_offset = 1;
            } else if self.x < next_knot.x {
                x_offset = -1;
            }
            let mut y_offset = 0;
            if self.y > next_knot.y {
                y_offset = 1;
            } else if self.y < next_knot.y {
                y_offset = -1;
            }
            next_knot.x += x_offset;
            next_knot.y += y_offset;
        }
    }

    fn adjust_tail(self: &Knot, knots: &mut Vec<Knot>) {
        let mut head = self.clone();
        for tail in knots.iter_mut() {
            head.adjust_following(tail);
            head = tail.clone();
        }
    }
}

pub fn day9() {
    let now = Instant::now();
    println!("Day 9 | Part 1: {:?}\nDay 9 | Part 2: {:?}", solution_1(), solution_2());
    println!("Took: {:?}", now.elapsed());
}

fn solution_1() -> usize {
    let head = Knot { x: 0, y: 0 };
    let tail = vec![Knot { x: 0, y: 0 }];

    move_head(head, tail)
}

fn solution_2() -> usize {
    let head = Knot { x: 0, y: 0 };
    let tail = vec![Knot { x: 0, y: 0 }; 9];

    move_head(head, tail)
}

fn move_head(mut head: Knot, mut tail: Vec<Knot>) -> usize {
    let mut visited: HashSet<Knot>  = HashSet::new();

    for mv in input_lines(9) {
        let distance = &mv[2..].parse::<i32>().unwrap();
        for _ in 0..*distance {
            match mv.chars().nth(0).unwrap() {
                'L' => { head.x -= 1; },
                'R' => { head.x += 1; },
                'U' => { head.y -= 1; },
                'D' => { head.y += 1; },
                _ => panic!("Invalid Input")
            }
            cascade_update(&head, &mut tail, &mut visited);
        }
    }
    visited.len()
}

fn cascade_update(head: &Knot, tail: &mut Vec<Knot>, visited: &mut HashSet<Knot>) {
    head.adjust_tail(tail);
    visited.insert(tail.last().unwrap().clone());
}