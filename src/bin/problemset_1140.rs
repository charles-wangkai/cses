use std::{
    collections::BTreeMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut p = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
        p.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a, &b, &p));
}

fn solve(a: &[i32], b: &[i32], p: &[i32]) -> i64 {
    let mut sorted_indices: Vec<_> = (0..a.len()).collect();
    sorted_indices.sort_by_key(|&i| b[i]);

    let mut end_day_to_money = BTreeMap::from([(0, 0)]);
    for index in sorted_indices {
        let money = end_day_to_money.range(..a[index]).next_back().unwrap().1 + (p[index] as i64);
        if money > *end_day_to_money.range(..=b[index]).next_back().unwrap().1 {
            end_day_to_money.insert(b[index], money);

            while let Some((&next_end_day, &next_money)) =
                end_day_to_money.range(b[index] + 1..).next()
            {
                if next_money > money {
                    break;
                }

                end_day_to_money.remove(&next_end_day);
            }
        }
    }

    *end_day_to_money.last_key_value().unwrap().1
}
