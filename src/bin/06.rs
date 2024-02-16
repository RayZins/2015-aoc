use itertools::Itertools;
use regex::Regex;

fn parse_range(line: &str) -> (usize, usize, usize, usize) {
    let pos = Regex::new(r"\d+").unwrap();
    pos.find_iter(line)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn main() {
    let input = include_str!("../inputs/06.txt");
    let mut part1 = [[false; 1000]; 1000];
    let mut part2 = [[0u32; 1000]; 1000];

    let re = Regex::new(r"on|off|toggle").unwrap();

    for line in input.lines() {
        let cmd = re.find(line).unwrap().as_str();
        let (x1, y1, x2, y2) = parse_range(line);

        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            part1[y][x] = match cmd {
                "on" => true,
                "off" => false,
                "toggle" => !part1[y][x],
                _ => unreachable!(),
            };
            part2[y][x] = part2[y][x].saturating_add_signed(match cmd {
                "on" => 1,
                "off" => -1,
                "toggle" => 2,
                _ => unreachable!(),
            });
        }
    }

    println!(
        "{} {}",
        part1.iter().flatten().filter(|elem| **elem).count(),
        part2.iter().flatten().sum::<u32>()
    );
}
