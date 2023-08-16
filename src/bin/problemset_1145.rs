use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x));
}

fn solve(x: &[i32]) -> usize {
    let mut values = Vec::new();
    for &xi in x {
        let index = find_first_larger_or_equal(&values, xi);
        if index == values.len() {
            values.push(xi);
        } else {
            values[index] = xi;
        }
    }

    values.len()
}

fn find_first_larger_or_equal(values: &[i32], target: i32) -> usize {
    let mut result = values.len();
    let mut lower = 0;
    let mut upper = (values.len() as i32) - 1;
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if values[middle as usize] >= target {
            result = middle as usize;
            upper = middle - 1;
        } else {
            lower = middle + 1;
        }
    }

    result
}
