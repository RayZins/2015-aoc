use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

fn calculate_happiness(metric: &HashMap<(&str, &str), i32>, attendees: &HashSet<&str>) -> i32 {
    let mut happiness = 0;
    let mut attendees = Vec::from_iter(attendees);

    // account for rotational symmetry
    let fixed = attendees.pop().unwrap();
    let num_attendees = attendees.len();

    for mut arrangement in attendees.iter().permutations(num_attendees) {
        arrangement.push(&fixed);
        happiness = max(
            happiness,
            arrangement
                .into_iter()
                .circular_tuple_windows()
                .map(|(&&a, &&b, &&c)| metric[&(b, a)] + metric[&(b, c)]) // && 🤔
                .sum(),
        );
    }

    happiness
}

fn main() {
    let input = include_str!("../inputs/13.txt");
    let mut happiness: HashMap<(&str, &str), i32> = HashMap::new();
    let mut guests: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let args: Vec<&str> = line.split_whitespace().collect();

        let who = args[0];
        let sign = if args[2] == "gain" { 1 } else { -1 };
        let points = args[3].parse::<i32>().unwrap();
        let beside = args[10].trim_end_matches('.');

        happiness.insert((who, beside), sign * points);
        guests.insert(who);
    }

    let part1 = calculate_happiness(&happiness, &guests);

    for g in &guests {
        happiness.insert(("You", g), 0);
        happiness.insert((g, "You"), 0);
    }
    guests.insert("You");

    let part2 = calculate_happiness(&happiness, &guests);

    println!("{} {}", part1, part2);
}
