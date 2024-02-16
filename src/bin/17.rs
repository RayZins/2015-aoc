use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/17.txt");
    let containers: Vec<u16> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut combinations = 0;
    let mut num_containers: HashMap<usize, u16> = HashMap::new();

    for mask in 0..(1 << containers.len()) as u32 {
        let mut cons: Vec<u16> = Vec::new();

        for bit in 0..(containers.len()) {
            if mask & (1 << bit) != 0 {
                cons.push(containers[bit]);
            }
        }

        if cons.iter().sum::<u16>() == 150 {
            combinations += 1;
            *num_containers.entry(cons.len()).or_default() += 1;
        };
    }

    println!("{}", combinations);
    println!(
        "{}",
        num_containers
            .iter()
            .min_by_key(|(k, _)| *k) // min # of containers
            .map(|(_, v)| v) // how many ways
            .unwrap()
    );
}
