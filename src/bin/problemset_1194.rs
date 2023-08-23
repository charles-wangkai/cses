use std::{
    collections::VecDeque,
    io::{stdin, BufRead, BufReader},
};

const OFFSETS: &[(char, i32, i32)] = &[('U', -1, 0), ('R', 0, 1), ('D', 1, 0), ('L', 0, -1)];

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

    let monster_distances = build_monster_distances(squares);

    let mut distances = vec![vec![i32::MAX; m]; n];
    let mut prevs = vec![vec![None; m]; n];
    let mut queue = VecDeque::new();
    let start = find_start(squares);
    distances[start.r as usize][start.c as usize] = 0;
    if distances[start.r as usize][start.c as usize]
        < monster_distances[start.r as usize][start.c as usize]
    {
        queue.push_back(start);
    }

    while let Some(head) = queue.pop_front() {
        if head.r == 0 || head.r == (n as i32) - 1 || head.c == 0 || head.c == (m as i32) - 1 {
            let mut path = String::new();
            let mut current = head;
            while let Some((motion, prev)) = prevs[current.r as usize][current.c as usize].clone() {
                path.push(motion);
                current = prev;
            }
            path = path.chars().rev().collect();

            return format!("YES\n{}\n{}", path.len(), path);
        }

        for &(motion, r_offset, c_offset) in OFFSETS {
            let adj_r = head.r + r_offset;
            let adj_c = head.c + c_offset;
            if (0..n as i32).contains(&adj_r)
                && (0..m as i32).contains(&adj_c)
                && squares[adj_r as usize][adj_c as usize] != '#'
                && distances[adj_r as usize][adj_c as usize] == i32::MAX
            {
                distances[adj_r as usize][adj_c as usize] =
                    distances[head.r as usize][head.c as usize] + 1;
                if distances[adj_r as usize][adj_c as usize]
                    < monster_distances[adj_r as usize][adj_c as usize]
                {
                    prevs[adj_r as usize][adj_c as usize] = Some((motion, head.clone()));
                    queue.push_back(Point { r: adj_r, c: adj_c });
                }
            }
        }
    }

    String::from("NO")
}

fn find_start(squares: &[Vec<char>]) -> Point {
    let mut r = 0;
    loop {
        for c in 0..squares[r].len() {
            if squares[r][c] == 'A' {
                return Point {
                    r: r as i32,
                    c: c as i32,
                };
            }
        }

        r += 1;
    }
}

fn build_monster_distances(squares: &[Vec<char>]) -> Vec<Vec<i32>> {
    let n = squares.len();
    let m = squares[0].len();

    let mut result = vec![vec![i32::MAX; m]; n];
    let mut queue = VecDeque::new();
    for r in 0..n {
        for c in 0..m {
            if squares[r][c] == 'M' {
                result[r][c] = 0;
                queue.push_back(Point {
                    r: r as i32,
                    c: c as i32,
                });
            }
        }
    }
    while let Some(head) = queue.pop_front() {
        for (_, r_offset, c_offset) in OFFSETS {
            let adj_r = head.r + r_offset;
            let adj_c = head.c + c_offset;
            if (0..n as i32).contains(&adj_r)
                && (0..m as i32).contains(&adj_c)
                && squares[adj_r as usize][adj_c as usize] != '#'
                && result[adj_r as usize][adj_c as usize] == i32::MAX
            {
                result[adj_r as usize][adj_c as usize] =
                    result[head.r as usize][head.c as usize] + 1;
                queue.push_back(Point { r: adj_r, c: adj_c });
            }
        }
    }

    result
}

#[derive(Clone)]
struct Point {
    r: i32,
    c: i32,
}
