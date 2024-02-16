use std::cell::RefCell; // 🫢
use std::collections::HashMap;

struct Circuit {
    instructions: HashMap<String, String>,
    memo: RefCell<HashMap<String, u16>>,
}

impl Circuit {
    fn new(input: &str) -> Self {
        Self {
            instructions: HashMap::from_iter(input.lines().map(|line| {
                let (i, o) = line.split_once(" -> ").unwrap();
                (o.to_string(), i.to_string())
            })),
            memo: RefCell::new(HashMap::new()),
        }
    }

    fn get_wire(&self, wire: &str) -> u16 {
        if let Some(v) = self.memo.borrow().get(wire) {
            return *v;
        }
        if let Ok(v) = wire.parse::<u16>() {
            return v;
        }

        let res: u16;
        let args: Vec<&str> = self
            .instructions
            .get(&wire.to_string())
            .unwrap()
            .split_whitespace()
            .collect();

        if args.len() == 1 {
            res = self.get_wire(&args[0])
        } else {
            res = match args[args.len() - 2] {
                "NOT" => !self.get_wire(&args[1]),
                "AND" => self.get_wire(&args[0]) & self.get_wire(&args[2]),
                "OR" => self.get_wire(&args[0]) | self.get_wire(&args[2]),
                "LSHIFT" => self.get_wire(&args[0]) << self.get_wire(&args[2]),
                "RSHIFT" => self.get_wire(&args[0]) >> self.get_wire(&args[2]),
                _ => unreachable!(),
            }
        }

        self.memo.borrow_mut().insert(wire.to_string(), res);
        res
    }
}

fn main() {
    let input = include_str!("../inputs/07.txt");
    let mut circuit = Circuit::new(input);

    let part1 = circuit.get_wire("a");

    circuit
        .instructions
        .insert("b".to_string(), part1.to_string());
    circuit.memo.borrow_mut().clear();

    let part2 = circuit.get_wire("a");
    println!("{} {}", part1, part2);
}
