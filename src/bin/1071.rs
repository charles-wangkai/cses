use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let t = split.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let y = split.next().unwrap().parse().unwrap();
        let x = split.next().unwrap().parse().unwrap();

        println!("{}", solve(y, x));
    }
}

fn solve(y: i32, x: i32) -> i64 {
    let size = y.max(x);

    ((size - 1) as i64).pow(2)
        + (if size % 2 == 0 {
            if y < size {
                y
            } else {
                2 * size - x
            }
        } else if x < size {
            x
        } else {
            2 * size - y
        } as i64)
}
