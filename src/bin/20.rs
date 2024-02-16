fn lowest_house_number(houses: Vec<u32>, num_presents: u32) -> Option<usize> {
    houses
        .iter()
        .enumerate()
        .find(|(_, &v)| v > num_presents)
        .map(|(i, _)| i)
}

fn main() {
    let input: u32 = include_str!("../inputs/20.txt").parse().unwrap();
    const UPPER_BOUND: usize = 1000000;
    let mut houses: Vec<u32> = vec![0; UPPER_BOUND];
    let mut houses2: Vec<u32> = vec![0; UPPER_BOUND];

    for i in 1..UPPER_BOUND {
        let mut j = i;
        while j < UPPER_BOUND {
            houses[j] += 10 * i as u32;
            j += i;
        }

        // part2
        let mut count = 0;
        let mut j = i;
        while count < 50 && j < UPPER_BOUND {
            houses2[j] += 11 * i as u32;
            j += i;
            count += 1;
        }
    }

    println!("{}", lowest_house_number(houses, input).unwrap());
    println!("{}", lowest_house_number(houses2, input).unwrap());
}
