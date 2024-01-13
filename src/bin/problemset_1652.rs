use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let q = split.next().unwrap().parse().unwrap();
    let mut squares = vec![vec![b'\0'; n]; n];
    for square_i in squares.iter_mut() {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let line: String = split.next().unwrap().parse().unwrap();
        square_i.copy_from_slice(line.as_bytes());
    }
    let mut y1s = Vec::new();
    let mut x1s = Vec::new();
    let mut y2s = Vec::new();
    let mut x2s = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        y1s.push(split.next().unwrap().parse().unwrap());
        x1s.push(split.next().unwrap().parse().unwrap());
        y2s.push(split.next().unwrap().parse().unwrap());
        x2s.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&squares, &y1s, &x1s, &y2s, &x2s));
}

fn solve(
    squares: &[Vec<u8>],
    y1s: &[usize],
    x1s: &[usize],
    y2s: &[usize],
    x2s: &[usize],
) -> String {
    let n = squares.len();

    let mut prefix_sums = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            prefix_sums[i][j] = prefix_sums[i - 1][j] + prefix_sums[i][j - 1]
                - prefix_sums[i - 1][j - 1]
                + (if squares[i - 1][j - 1] == b'*' { 1 } else { 0 });
        }
    }

    (0..y1s.len())
        .map(|i| {
            prefix_sums[y2s[i]][x2s[i]]
                - prefix_sums[y1s[i] - 1][x2s[i]]
                - prefix_sums[y2s[i]][x1s[i] - 1]
                + prefix_sums[y1s[i] - 1][x1s[i] - 1]
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
