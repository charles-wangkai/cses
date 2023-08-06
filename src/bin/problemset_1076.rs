use std::{
    collections::BTreeSet,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let k = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, k));
}

fn solve(x: &[i32], k: i32) -> String {
    let mut result = Vec::new();
    let mut lower_elements = BTreeSet::new();
    let mut upper_elements = BTreeSet::new();
    for i in 0..k - 1 {
        lower_elements.insert((x[i as usize], i));
    }
    balance_lower_and_upper(&mut lower_elements, &mut upper_elements);

    for i in k - 1..x.len() as i32 {
        lower_elements.insert((x[i as usize], i));
        upper_elements.insert(lower_elements.pop_last().unwrap());
        balance_lower_and_upper(&mut lower_elements, &mut upper_elements);

        result.push(lower_elements.last().unwrap().0);

        let element = (x[(i - k + 1) as usize], i - k + 1);
        if !lower_elements.remove(&element) {
            upper_elements.remove(&element);
        }
        balance_lower_and_upper(&mut lower_elements, &mut upper_elements);
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn balance_lower_and_upper(
    lower_elements: &mut BTreeSet<(i32, i32)>,
    upper_elements: &mut BTreeSet<(i32, i32)>,
) {
    while lower_elements.len() > upper_elements.len() + 1 {
        upper_elements.insert(lower_elements.pop_last().unwrap());
    }
    while lower_elements.len() < upper_elements.len() {
        lower_elements.insert(upper_elements.pop_first().unwrap());
    }
}
