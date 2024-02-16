use std::cmp::max;

fn main() {
    // let input = include_str!("../inputs/15.txt");
    let mut score = 0;

    for a in 0..100_u32 {
        for b in 0..(100 - a) {
            for c in 0..(100 - a - b) {
                let d = 100 - a - b - c;

                // part 2
                let cal = (2 * a) + (9 * b) + c + (8 * d);
                if cal != 500 {
                    continue;
                }

                let cap = (3 * a).saturating_sub(3 * b).saturating_sub(c);
                let dur = 3 * b;
                let fla = (4 * c).saturating_sub(2 * d);
                let tex = (2 * d).saturating_sub(3 * a);

                score = max(score, cap * dur * fla * tex);
            }
        }
    }

    println!("{}", score);
}
