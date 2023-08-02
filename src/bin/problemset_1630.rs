use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut d = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        d.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&mut a, &d));
}

fn solve(a: &mut [i32], d: &[i32]) -> i64 {
    a.sort_by_key(|x| -x);

    d.iter().map(|&di| di as i64).sum::<i64>()
        - (0..a.len())
            .map(|i| ((i as i64) + 1) * (a[i] as i64))
            .sum::<i64>()
}
