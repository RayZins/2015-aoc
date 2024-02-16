fn main() {
    let input = include_str!("../inputs/01.txt");

    let arr: Vec<i16> = input
        .chars()
        .scan(0, |floor, c| {
            *floor += if c == '(' { 1 } else { -1 };
            Some(*floor)
        })
        .collect();

    let part1 = arr.last().unwrap();
    let part2 = arr.iter().position(|&floor| floor == -1).unwrap() + 1;
    println!("{:?} {:?}", part1, part2);
}
