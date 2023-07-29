use std::io::{stdin, BufRead, BufReader};

const LIMIT: i64 = 100_000_000_000_000_000;

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let q = split.next().unwrap().parse().unwrap();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let k = split.next().unwrap().parse().unwrap();

        println!("{}", solve(k));
    }
}

fn solve(k: i64) -> u8 {
    let prev_value = find_prev_value(k);

    ((prev_value + 1)
        .to_string()
        .chars()
        .nth((k - compute_digit_num(prev_value) - 1) as usize)
        .unwrap() as u8)
        - b'0'
}

fn find_prev_value(k: i64) -> i64 {
    let mut result = -1;
    let mut lower = 0;
    let mut upper = LIMIT;
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if compute_digit_num(middle) < k {
            result = middle;
            lower = middle + 1;
        } else {
            upper = middle - 1;
        }
    }

    result
}

fn compute_digit_num(n: i64) -> i64 {
    let mut result = 0;
    let mut length = 1;
    let mut power10 = 1;
    while n >= power10 * 10 {
        result += 9 * power10 * length;

        length += 1;
        power10 *= 10;
    }
    result += (n - power10 + 1) * length;

    result
}
