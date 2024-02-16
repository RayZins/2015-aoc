use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/18.txt");
    let mut lights: HashSet<(i8, i8)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, elem) in line.chars().enumerate() {
            if elem == '#' {
                lights.insert((y as i8, x as i8));
            }
        }
    }

    for _step in 0..100 {
        lights = (0..100)
            .cartesian_product(0..100)
            .filter(|(y, x)| {
                let nbrs_on = (-1..2)
                    .cartesian_product(-1..2)
                    .filter(|it| it != &(0, 0))
                    .filter(|(dy, dx)| lights.contains(&(y + dy, x + dx)))
                    .count();

                lights.contains(&(*y, *x)) && (2..=3).contains(&nbrs_on) || nbrs_on == 3
            })
            .collect();

        // part 2
        lights.extend([(0, 0), (0, 99), (99, 0), (99, 99)]);
    }

    println!("{:?}", lights.len());
}
