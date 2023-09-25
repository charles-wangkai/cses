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
    let m = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
        c.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b, &c));
}

fn solve(n: usize, a: &[usize], b: &[usize], c: &[i32]) -> String {
    let mut distances = vec![0; n];
    let mut prevs = vec![usize::MAX; n];

    let mut round = 0;
    loop {
        let mut last = usize::MAX;
        let mut updated = false;
        for i in 0..a.len() {
            if distances[a[i] - 1] + (c[i] as i64) < distances[b[i] - 1] {
                distances[b[i] - 1] = distances[a[i] - 1] + (c[i] as i64);
                prevs[b[i] - 1] = a[i] - 1;
                last = b[i] - 1;
                updated = true;
            }
        }

        if !updated {
            break;
        }

        if round == n - 1 {
            let mut cycle = vec![last];
            let mut seen = HashSet::from([last]);
            loop {
                last = prevs[last];
                cycle.push(last);
                if seen.contains(&last) {
                    let mut index = 0;
                    while cycle[index] != last {
                        index += 1;
                    }

                    cycle = Vec::from(&cycle[index..]);

                    break;
                }

                seen.insert(last);
            }

            return format!(
                "YES\n{}",
                cycle
                    .iter()
                    .rev()
                    .map(|x| x + 1)
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        round += 1;
    }

    String::from("NO")
}
