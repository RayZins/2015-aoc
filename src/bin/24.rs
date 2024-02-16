// somehow this works, and checking that the remaining packages could be summed up to 2 (3) groups is not required??

use itertools::Itertools;

fn try_group(packages: Vec<usize>, required_weight: usize) -> usize {
    for len in 5..packages.len() {
        if let Some(qe) = packages
            .iter()
            .combinations(len)
            .filter(|c| c.iter().copied().sum::<usize>() == required_weight)
            .map(|p| p.iter().fold(1, |acc, &x| acc * x))
            .min()
        {
            return qe;
        }
    }
    0
}

fn main() {
    let input = include_str!("../inputs/24.txt");

    let packages: Vec<usize> = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    // let required_weight = packages.iter().sum::<usize>() / 3;
    let required_weight = packages.iter().sum::<usize>() / 4; // part2

    println!("{}", try_group(packages, required_weight));
}
