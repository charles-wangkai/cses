use std::{
    collections::VecDeque,
    io::{stdin, BufRead, BufReader},
};

const OFFSETS: &[(i32, i32)] = &[
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();

    println!("{}", solve(n));
}

fn solve(n: i32) -> String {
    let mut result = vec![vec![None; n as usize]; n as usize];
    result[0][0] = Some(0);

    let mut queue = VecDeque::new();
    queue.push_back(Point { r: 0, c: 0 });

    while let Some(head) = queue.pop_front() {
        for (r_offset, c_offset) in OFFSETS {
            let adj_r = head.r + r_offset;
            let adj_c = head.c + c_offset;
            if adj_r >= 0
                && adj_r < n
                && adj_c >= 0
                && adj_c < n
                && result[adj_r as usize][adj_c as usize].is_none()
            {
                result[adj_r as usize][adj_c as usize] =
                    Some(result[head.r as usize][head.c as usize].unwrap() + 1);
                queue.push_back(Point { r: adj_r, c: adj_c });
            }
        }
    }

    result
        .iter()
        .map(|line| {
            line.iter()
                .map(|x| x.unwrap().to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

struct Point {
    r: i32,
    c: i32,
}
