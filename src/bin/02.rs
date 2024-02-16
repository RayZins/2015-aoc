struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl TryFrom<&str> for Present {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut nums = value.splitn(3, "x").map(|n| n.parse::<u32>().unwrap());
        let l = nums.next().unwrap();
        let w = nums.next().unwrap();
        let h = nums.next().unwrap();
        Ok(Present {
            length: l,
            width: w,
            height: h,
        })
    }
}

impl Present {
    fn surfaces(&self) -> impl Iterator<Item = u32> {
        [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
        .into_iter()
    }

    fn sides(&self) -> impl Iterator<Item = u32> {
        [
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.height + self.length),
        ]
        .into_iter()
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }
}

fn main() {
    let input = include_str!("../inputs/02.txt");

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    for line in input.lines() {
        let present: Present = line.try_into().unwrap();
        part1 += 2 * present.surfaces().sum::<u32>() + present.surfaces().min().unwrap();
        part2 += present.volume() + present.sides().min().unwrap();
    }

    println!("{part1} {part2}");
}
