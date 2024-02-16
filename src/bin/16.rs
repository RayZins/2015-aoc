use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../inputs/16.txt");
    let aunts: Vec<HashMap<&str, u8>> = input
        .lines()
        .map(|line| {
            HashMap::from_iter(line.split_whitespace().skip(2).chunks(2).into_iter().map(
                |mut chunk| {
                    let (k, v) = chunk.next_tuple().unwrap();
                    (
                        k.trim_matches(':'),
                        v.trim_matches(',').parse::<u8>().unwrap(),
                    )
                },
            ))
        })
        .collect();

    let mfcsam = "children: 3
                        cats: 7
                        samoyeds: 2
                        pomeranians: 3
                        akitas: 0
                        vizslas: 0
                        goldfish: 5
                        trees: 3
                        cars: 2
                        perfumes: 1";
    let mfcsam: HashMap<&str, u8> = HashMap::from_iter(mfcsam.lines().map(|line| {
        let (k, v) = line.split_once(": ").unwrap();
        (k.trim(), v.parse::<u8>().unwrap())
    }));

    for (n, sue) in aunts.iter().enumerate() {
        if sue.iter().all(|(compound, qty)| *qty == mfcsam[compound]) {
            println!("{}", n + 1);
        }
        if sue.iter().all(|(compound, qty)| {
            let compare = match *compound {
                "cats" | "trees" => std::cmp::PartialOrd::gt,
                "pomeranians" | "goldfish" => std::cmp::PartialOrd::lt,
                _ => std::cmp::PartialEq::eq,
            };
            compare(qty, &mfcsam[compound])
        }) {
            println!("{}", n + 1);
        }
    }
}
