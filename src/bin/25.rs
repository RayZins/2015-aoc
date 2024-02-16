fn operate(num: usize) -> usize {
    num * 252533 % 33554393
}

fn main() {
    // let input = include_str!("../inputs/25.txt");
    let (row, col) = (3010, 3019);

    let mut start = 20151125;
    let num_of_operations = (col * (col + 1) / 2) + ((row - 1) * (2 * col + row - 2) / 2);

    for _ in 1..num_of_operations {
        start = operate(start);
    }

    println!("{}", start);
}
