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
    let mut sorted_indices: Vec<_> = (0..a.len()).collect();
    sorted_indices.sort_by_key(|&i| c[i]);

    let mut result = 0;
    let mut parents = vec![usize::MAX; n];
    for index in sorted_indices {
        let root1 = find_root(&mut parents, a[index] - 1);
        let root2 = find_root(&mut parents, b[index] - 1);
        if root1 != root2 {
            parents[root2] = root1;
            result += c[index] as i64;
        }
    }

    if (0..n)
        .map(|i| find_root(&mut parents, i))
        .collect::<HashSet<_>>()
        .len()
        == 1
    {
        result.to_string()
    } else {
        String::from("IMPOSSIBLE")
    }
}

fn find_root(parents: &mut [usize], node: usize) -> usize {
    if parents[node] == usize::MAX {
        return node;
    }

    parents[node] = find_root(parents, parents[node]);

    parents[node]
}
