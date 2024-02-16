// conway cosmological seq

use itertools::Itertools;

fn looksay(seq: &str) -> String {
    let mut tr = String::new();
    for (key, grp) in &seq.chars().group_by(|elem| *elem) {
        tr.push_str(&format!("{}{}", grp.count(), key));
    }
    tr
}

fn main() {
    let mut input = include_str!("../inputs/10.txt").to_string();

    for _ in 0..40 {
        // part2 0..50
        input = looksay(&input);
    }

    println!("{}", input.len());
}
