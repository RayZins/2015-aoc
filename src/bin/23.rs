use std::str::FromStr;

type Offset = i8;
type R = usize;

#[derive(Debug)]
enum Instruction {
    Hlf(R),
    Tpl(R),
    Inc(R),
    Jmp(Offset),
    Jie((R, Offset)),
    Jio((R, Offset)),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(" ").unwrap();

        fn to_reg(c: &str) -> R {
            c.trim().parse::<char>().unwrap() as usize - 'a' as usize
        }

        fn num_parse(num: &str) -> Offset {
            num.trim_start_matches('+').parse().unwrap()
        }

        fn ji_helper(rhs: &str) -> (R, Offset) {
            let (r, o) = rhs.split_once(", ").unwrap();
            (to_reg(r), num_parse(o))
        }

        let inst = match x {
            "hlf" => Instruction::Hlf(to_reg(y)),
            "tpl" => Instruction::Tpl(to_reg(y)),
            "inc" => Instruction::Inc(to_reg(y)),
            "jmp" => Instruction::Jmp(num_parse(y)),
            "jie" => Instruction::Jie(ji_helper(y)),
            "jio" => Instruction::Jio(ji_helper(y)),
            _ => return Err(()),
        };

        Ok(inst)
    }
}

fn main() {
    let input = include_str!("../inputs/23.txt");

    // let mut registers = [0, 0];
    let mut registers: [isize; 2] = [1, 0]; // part2
    let mut ip = 0;

    let stack: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();
    while let Some(inst) = stack.get(ip) {
        match inst {
            Instruction::Hlf(idx) => *registers.get_mut(*idx).unwrap() /= 2,
            Instruction::Tpl(idx) => *registers.get_mut(*idx).unwrap() *= 3,
            Instruction::Inc(idx) => *registers.get_mut(*idx).unwrap() += 1,
            Instruction::Jmp(offset) => {
                ip = ip.saturating_add_signed(*offset as isize);
                continue;
            }
            Instruction::Jie((idx, offset)) => {
                if registers[*idx] % 2 == 0 {
                    ip = ip.saturating_add_signed(*offset as isize);
                    continue;
                }
            }
            Instruction::Jio((idx, offset)) => {
                if registers[*idx] == 1 {
                    ip = ip.saturating_add_signed(*offset as isize);
                    continue;
                }
            }
        }
        ip += 1;
    }

    println!("{:?}", registers);
}
