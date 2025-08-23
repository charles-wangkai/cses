use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();

    println!("{}", solve(n));
}

fn solve(n: usize) -> String {
    let mut result = vec![vec![0; n]; n];
    let mut row_seens = vec![HashSet::new(); n];
    let mut col_seens = vec![HashSet::new(); n];

    for (r, row_seen) in row_seens.iter_mut().enumerate() {
        for (c, col_seen) in col_seens.iter_mut().enumerate() {
            while row_seen.contains(&result[r][c]) || col_seen.contains(&result[r][c]) {
                result[r][c] += 1;
            }

            row_seen.insert(result[r][c]);
            col_seen.insert(result[r][c]);
        }
    }

    result
        .iter()
        .map(|line| {
            line.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
