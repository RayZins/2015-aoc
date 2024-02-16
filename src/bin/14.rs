use itertools::{repeat_n, Itertools};
use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/14.txt");
    let mut logs: HashMap<&str, Vec<u32>> = HashMap::new();

    for line in input.lines() {
        let args: Vec<&str> = line.split_whitespace().collect();

        let name = args[0];
        let speed = args[3].parse::<u32>().unwrap();
        let sustain = args[6].parse::<usize>().unwrap();
        let slp = args[13].parse::<usize>().unwrap();

        let flying = repeat_n(speed, sustain);
        let resting = repeat_n(0, slp);
        let work_rest = flying.chain(resting);

        logs.insert(
            name,
            work_rest
                .cycle()
                .take(2503)
                .scan(0, |dist, spd| {
                    *dist += spd;
                    Some(*dist)
                })
                .collect(),
        );
    }

    println!(
        "{}",
        logs.iter()
            .max_by_key(|(_, v)| v.last())
            .map(|(_, v)| v.last())
            .unwrap()
            .unwrap()
    );

    let mut wins: HashMap<&str, usize> = HashMap::new();
    for i in 0..2503 {
        let dists: Vec<(&str, u32)> = logs.keys().map(|&deer| (deer, logs[deer][i])).collect();
        for winner in dists
            .iter()
            .max_set_by_key(|(_, dist)| dist)
            .iter()
            .map(|(name, _)| name)
        {
            *wins.entry(winner).or_default() += 1;
        }
    }

    println!(
        "{:?}",
        wins.iter().max_by_key(|(_, v)| *v).map(|(_, v)| v).unwrap()
    );
}
