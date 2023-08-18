use std::{
    collections::VecDeque,
    io::{stdin, BufRead, BufReader},
};

const OFFSETS: &[(char, i32, i32)] = &[('D', -1, 0), ('L', 0, 1), ('U', 1, 0), ('R', 0, -1)];

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    split.next().unwrap();
    let mut squares = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        let line: String = split.next().unwrap().parse().unwrap();
        squares.push(line.chars().collect());
    }

    println!("{}", solve(&squares));
}

fn solve(squares: &[Vec<char>]) -> String {
    let n = squares.len();
    let m = squares[0].len();

    let end = find(squares, 'B');

    let mut nexts = vec![vec![None; m]; n];
    nexts[end.r as usize][end.c as usize] = Some(('\0', Point { r: 0, c: 0 }));

    let mut queue = VecDeque::new();
    queue.push_back(end.clone());

    while let Some(head) = queue.pop_front() {
        for &(motion, r_offset, c_offset) in OFFSETS {
            let adj_r = head.r + r_offset;
            let adj_c = head.c + c_offset;
            if (0..n as i32).contains(&adj_r)
                && (0..m as i32).contains(&adj_c)
                && squares[adj_r as usize][adj_c as usize] != '#'
                && nexts[adj_r as usize][adj_c as usize].is_none()
            {
                nexts[adj_r as usize][adj_c as usize] = Some((motion, head.clone()));
                queue.push_back(Point { r: adj_r, c: adj_c });
            }
        }
    }

    let start = find(squares, 'A');
    if nexts[start.r as usize][start.c as usize].is_none() {
        return String::from("NO");
    }

    let mut path = String::new();
    let mut current = start;
    while current != end {
        let (motion, next) = nexts[current.r as usize][current.c as usize]
            .clone()
            .unwrap();
        path.push(motion);
        current = next;
    }

    format!("YES\n{}\n{}", path.len(), path)
}

fn find(squares: &[Vec<char>], target: char) -> Point {
    let mut r = 0;
    loop {
        for c in 0..squares[r].len() {
            if squares[r][c] == target {
                return Point {
                    r: r as i32,
                    c: c as i32,
                };
            }
        }

        r += 1;
    }
}

#[derive(Clone, PartialEq)]
struct Point {
    r: i32,
    c: i32,
}
