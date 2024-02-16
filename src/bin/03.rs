use std::collections::HashSet;
use std::mem::swap;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn update(&mut self, dir: char) {
        match dir {
            '^' => self.1 += 1,
            'v' => self.1 -= 1,
            '<' => self.0 -= 1,
            '>' => self.0 += 1,
            _ => panic!("unknown instruction"),
        }
    }
}

fn main() {
    let input = include_str!("../inputs/03.txt");

    let mut visited: HashSet<Point> = HashSet::new();
    let mut santa = Point(0, 0);
    let mut robo = Point(0, 0);
    visited.insert(santa);

    for dir in input.chars() {
        santa.update(dir);
        visited.insert(santa);
        // part2
        swap(&mut santa, &mut robo);
    }

    println!("{}", visited.len());
}
