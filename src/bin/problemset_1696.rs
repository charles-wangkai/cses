// https://www.cnblogs.com/czsharecode/p/9777533.html

use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let k = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..k {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, m, &a, &b));
}

fn solve(n: usize, m: usize, a: &[usize], b: &[usize]) -> String {
    let mut left_adj_vecs = vec![Vec::new(); n];
    for i in 0..a.len() {
        left_adj_vecs[a[i] - 1].push(b[i] - 1);
    }

    let mut left_matchings = vec![usize::MAX; n];
    let mut right_matchings = vec![usize::MAX; m];
    for i in 0..left_matchings.len() {
        if left_matchings[i] == usize::MAX {
            search(
                &left_adj_vecs,
                &mut left_matchings,
                &mut right_matchings,
                &mut vec![false; m],
                i,
            );
        }
    }

    let mut pairs = Vec::new();
    for (i, &left_matchings_i) in left_matchings.iter().enumerate() {
        if left_matchings_i != usize::MAX {
            pairs.push(format!("{} {}", i + 1, left_matchings_i + 1));
        }
    }

    format!("{}\n{}", pairs.len(), pairs.join("\n"))
}

fn search(
    left_adj_vecs: &[Vec<usize>],
    left_matchings: &mut [usize],
    right_matchings: &mut [usize],
    right_visited: &mut [bool],
    left_index: usize,
) -> bool {
    for &right_index in &left_adj_vecs[left_index] {
        if !right_visited[right_index] {
            right_visited[right_index] = true;

            if right_matchings[right_index] == usize::MAX
                || search(
                    left_adj_vecs,
                    left_matchings,
                    right_matchings,
                    right_visited,
                    right_matchings[right_index],
                )
            {
                left_matchings[left_index] = right_index;
                right_matchings[right_index] = left_index;

                return true;
            }
        }
    }

    false
}
