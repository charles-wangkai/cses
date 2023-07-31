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
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut k = Vec::new();
    for _ in 0..n {
        k.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&k));
}

fn solve(k: &[i32]) -> i32 {
    let mut result = 0;
    let mut seen = HashSet::new();
    let mut right_index = -1;
    for i in 0..k.len() {
        while right_index != (k.len() as i32) - 1 && !seen.contains(&k[(right_index + 1) as usize])
        {
            seen.insert(k[(right_index + 1) as usize]);
            right_index += 1;
        }

        result = result.max(right_index - (i as i32) + 1);
        seen.remove(&k[i]);
    }

    result
}
