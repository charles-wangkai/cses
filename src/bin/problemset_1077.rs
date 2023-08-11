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
    let mut lower_sum = 0;
    let mut upper_elements = BTreeSet::new();
    let mut upper_sum = 0;
    for i in 0..k - 1 {
        lower_elements.insert((x[i as usize], i));
        lower_sum += x[i as usize] as i64;
    }
    balance_lower_and_upper(
        &mut lower_elements,
        &mut lower_sum,
        &mut upper_elements,
        &mut upper_sum,
    );

    for i in k - 1..x.len() as i32 {
        lower_elements.insert((x[i as usize], i));
        lower_sum += x[i as usize] as i64;

        let element = lower_elements.pop_last().unwrap();
        lower_sum -= element.0 as i64;

        upper_elements.insert(element);
        upper_sum += element.0 as i64;

        balance_lower_and_upper(
            &mut lower_elements,
            &mut lower_sum,
            &mut upper_elements,
            &mut upper_sum,
        );

        result.push(
            upper_sum
                - (lower_sum
                    - if lower_elements.len() == upper_elements.len() + 1 {
                        lower_elements.last().unwrap().0 as i64
                    } else {
                        0
                    }),
        );

        let element = (x[(i - k + 1) as usize], i - k + 1);
        if lower_elements.contains(&element) {
            lower_elements.remove(&element);
            lower_sum -= element.0 as i64;
        } else {
            upper_elements.remove(&element);
            upper_sum -= element.0 as i64;
        }
        balance_lower_and_upper(
            &mut lower_elements,
            &mut lower_sum,
            &mut upper_elements,
            &mut upper_sum,
        );
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn balance_lower_and_upper(
    lower_elements: &mut BTreeSet<(i32, i32)>,
    lower_sum: &mut i64,
    upper_elements: &mut BTreeSet<(i32, i32)>,
    upper_sum: &mut i64,
) {
    while lower_elements.len() > upper_elements.len() + 1 {
        let element = lower_elements.pop_last().unwrap();
        *lower_sum -= element.0 as i64;

        upper_elements.insert(element);
        *upper_sum += element.0 as i64;
    }
    while lower_elements.len() < upper_elements.len() {
        let element = upper_elements.pop_first().unwrap();
        *upper_sum -= element.0 as i64;

        lower_elements.insert(element);
        *lower_sum += element.0 as i64;
    }
}
