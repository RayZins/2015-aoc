use itertools::Itertools;
use regex::Regex;

fn moves(attacker: (i32, i32, i32), defender: (i32, i32, i32)) -> i32 {
    let a = defender.0;
    let b = 1.max(attacker.1 - defender.2);

    a / b + (a % b).signum() // round up
}

fn main() {
    let input = include_str!("../inputs/21.txt");
    let re = Regex::new(r"\d+").unwrap();

    let boss: (i32, i32, i32) = input
        .lines()
        .map(|line| re.find(line).unwrap().as_str())
        .map(|m| m.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();

    let weapons: &[(i32, i32, i32)] = &[(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armour: &[(i32, i32, i32)] = &[
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    let rings: &[(i32, i32, i32)] = &[
        (0, 0, 0),
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let mut min_cost = i32::MAX;
    let mut max_cost = 0;
    for ((w, a), (r1, r2)) in weapons
        .iter()
        .cartesian_product(armour.iter())
        .cartesian_product(rings.iter().tuple_combinations())
    {
        let cost = w.0 + a.0 + r1.0 + r2.0;
        let dmg = w.1 + r1.1 + r2.1;
        let amr = a.2 + r1.2 + r2.2;

        let player = (100, dmg, amr);

        if moves(player, boss) <= moves(boss, player) {
            min_cost = min_cost.min(cost);
        } else {
            max_cost = max_cost.max(cost);
        }
    }

    println!("{} {}", min_cost, max_cost);
}
