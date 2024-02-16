use regex::Regex;

fn main() {
    let input = include_str!("../inputs/08.txt");
    let re = Regex::new(r#"\\x[[:xdigit:]]{2}|\\\\|\\""#).unwrap();

    let part1 = input
        .lines()
        .map(|line| {
            let new = re.replace_all(&line, "_");
            line.len() - (new.len() - 2)
        })
        .sum::<usize>();

    let part2 = input
        .lines()
        .map(|line| {
            let new = line.replace(r"\", r"\\").replace(r#"""#, r#"\""#);
            (new.len() + 2) - line.len()
        })
        .sum::<usize>();

    println!("{} {}", part1, part2);
}
