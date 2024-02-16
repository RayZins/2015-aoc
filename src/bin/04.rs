use md5;

fn main() {
    let input = include_str!("../inputs/04.txt");
    let mut answer = 0;

    loop {
        let digest = md5::compute(format!("{input}{answer}")).0;
        // if digest[..2] == [0, 0] && digest[2] <= 0x0F {
        //     break;
        // }

        // part 2
        if digest[..3] == [0, 0, 0] {
            break;
        }
        answer += 1;
    }

    println!("{answer}");
}
