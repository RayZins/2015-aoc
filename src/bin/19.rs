use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let (replacements, medicine) = include_str!("../inputs/19.txt").split_once("\n\n").unwrap();
    let mut distinct: HashSet<String> = HashSet::new();
    let mut part2_replacements: HashMap<&str, &str> = HashMap::new();

    for line in replacements.lines() {
        let (frm, to) = line.split_once(" => ").unwrap();
        part2_replacements.insert(to, frm);
        let re = Regex::new(frm).unwrap();

        for m in re.find_iter(medicine) {
            distinct.insert(format!(
                "{}{}{}",
                &medicine[..m.start()],
                to,
                &medicine[m.end()..]
            ));
        }
    }

    println!("{}", distinct.len());

    // part 2 bruteforce somehow worked due to HashMap not having order?
    'outer: loop {
        let mut steps = 0;
        let mut reduced = String::from(medicine);

        while reduced != "e" {
            if part2_replacements.keys().all(|k| !reduced.contains(k)) {
                break;
            }

            for (frm, to) in &part2_replacements {
                if !reduced.contains(frm) {
                    continue;
                }

                reduced = reduced.replacen(frm, to, 1);
                steps += 1;

                if reduced == "e" {
                    println!("{}", steps);
                    break 'outer;
                }
            }
        }
    }
}
