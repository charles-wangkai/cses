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

fn solve(x: &[i32]) -> String {
    let mut dp = vec![false; (x.iter().sum::<i32>() as usize) + 1];
    dp[0] = true;
    for &xi in x {
        for i in (xi as usize..dp.len()).rev() {
            dp[i] |= dp[i - (xi as usize)];
        }
    }

    let sums: Vec<_> = (1..dp.len()).filter(|&i| dp[i]).collect();

    format!(
        "{}\n{}",
        sums.len(),
        sums.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
