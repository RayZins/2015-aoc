use std::str;

fn main() {
    let input = include_str!("../inputs/05.txt");

    let part1 = input
        .lines()
        .filter(|line| {
            let vowels = line.chars().filter(|c| "aeiou".contains(*c));
            let has_contiguous = line.as_bytes().windows(2).any(|twice| twice[0] == twice[1]);
            let no_blacklist = ["ab", "cd", "pq", "xy"]
                .iter()
                .all(|&bl| !line.contains(bl));

            vowels.count() >= 3 && has_contiguous && no_blacklist
        })
        .count();

    let part2 = input
        .lines()
        .filter(|&line| {
            let has_pair = line
                .as_bytes()
                .windows(2)
                .any(|pair| line.matches(str::from_utf8(pair).unwrap()).count() >= 2);
            let repeats = line
                .as_bytes()
                .windows(3)
                .any(|window| window[0] == window[2]);

            has_pair && repeats
        })
        .count();

    println!("{} {}", part1, part2);
}
