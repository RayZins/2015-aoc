use itertools::Itertools;
use std::{fmt::Display, ops::Add, str};

#[derive(Debug)]
struct Password {
    letters: [u8; 8],
}

impl Password {
    fn new(pw: &str) -> Self {
        assert!(pw.chars().all(|c| c.is_lowercase()));
        Self {
            letters: pw
                .bytes()
                .map(|c| c - b'a')
                .collect::<Vec<u8>>() // owells
                .try_into()
                .unwrap(),
        }
    }
    fn includes_increasing(&self) -> bool {
        self.letters
            .into_iter()
            .tuple_windows()
            .any(|(a, b, c)| c.saturating_sub(b) == 1 && b.saturating_sub(a) == 1)
    }
    fn contains_iol(&self) -> bool {
        self.letters
            .into_iter()
            .any(|c| c == b'i' - b'a' || c == b'o' - b'a' || c == b'l' - b'a')
    }
    fn contains_pairs(&self) -> bool {
        let pairs: Vec<&[u8]> = self
            .letters
            .windows(2)
            .filter(|pair| pair[0] == pair[1])
            .collect();

        pairs.len() >= 2 && pairs[0][0] != pairs[1][1]
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            str::from_utf8(&self.letters.iter().map(|c| c + b'a').collect::<Vec<u8>>()).unwrap()
        )
    }
}

impl Add<u8> for Password {
    type Output = Self;

    fn add(mut self, rhs: u8) -> Self::Output {
        self.letters[7] += rhs;
        for (a, b) in (0..8usize).rev().tuple_windows() {
            let val = self.letters[a];
            if val > 25 {
                self.letters[b] += val / 26;
                self.letters[a] = val % 26;
            }
        }
        self
    }
}

fn next_pw(mut pw: Password) -> Password {
    loop {
        pw = pw + 1;
        if !pw.contains_iol() && pw.includes_increasing() && pw.contains_pairs() {
            return pw;
        }
    }
}

fn main() {
    let input = include_str!("../inputs/11.txt");
    let pwd = Password::new(input);

    let part1 = next_pw(pwd);
    println!("{}", &part1);

    let part2 = next_pw(part1);
    println!("{}", part2);
}
