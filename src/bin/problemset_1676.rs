use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let m = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(n, &a, &b));
}

fn solve(n: usize, a: &[usize], b: &[usize]) -> String {
    let mut component_nums = vec![usize::MAX; a.len()];
    let mut max_component_sizes = vec![-1; a.len()];
    let mut parents = vec![usize::MAX; n];
    let mut sizes = vec![1; n];
    let mut component_num = n;
    let mut max_component_size = 1;
    for i in 0..a.len() {
        let root1 = find_root(&mut parents, a[i] - 1);
        let root2 = find_root(&mut parents, b[i] - 1);
        if root1 != root2 {
            parents[root2] = root1;
            sizes[root1] += sizes[root2];
            component_num -= 1;
            max_component_size = max_component_size.max(sizes[root1]);
        }

        component_nums[i] = component_num;
        max_component_sizes[i] = max_component_size;
    }

    (0..a.len())
        .map(|i| format!("{} {}", component_nums[i], max_component_sizes[i]))
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_root(parents: &mut [usize], node: usize) -> usize {
    if parents[node] == usize::MAX {
        return node;
    }

    parents[node] = find_root(parents, parents[node]);

    parents[node]
}
