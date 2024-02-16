// TSP

use itertools::Itertools;
use std::cmp::{max, min};

fn main() {
    // let input = include_str!("../inputs/09.txt");
    // hardcoded the adjacency matrix 😂
    let mut shortest = usize::MAX;
    let mut longest = 0;
    let adj = [
        [0, 34, 100, 63, 108, 111, 89, 132],
        [34, 0, 4, 79, 44, 147, 133, 74],
        [100, 4, 0, 105, 95, 48, 88, 7],
        [63, 79, 105, 0, 68, 134, 107, 40],
        [108, 44, 95, 68, 0, 11, 66, 144],
        [111, 147, 48, 134, 11, 0, 115, 135],
        [89, 133, 88, 107, 66, 115, 0, 127],
        [132, 74, 7, 40, 144, 135, 127, 0],
    ];

    let possible = (0..=7u8).permutations(8);
    for path in possible {
        let dist = path
            .into_iter()
            .tuple_windows()
            .map(|(src, dst)| adj[src as usize][dst as usize])
            .sum();
        shortest = min(shortest, dist);
        longest = max(longest, dist);
    }

    println!("{} {}", shortest, longest);
}
