// Page 51 in https://cses.fi/book/book.pdf

use std::io::{stdin, BufRead, BufReader};

const SIZE: usize = 7;
const OFFSETS: &[(char, i32, i32)] = &[('U', -1, 0), ('R', 0, 1), ('D', 1, 0), ('L', 0, -1)];

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let s: String = split.next().unwrap().parse().unwrap();

    println!("{}", solve(&s));
}

fn solve(s: &str) -> i32 {
    search(
        &s.chars().collect::<Vec<_>>(),
        &mut [[false; SIZE]; SIZE],
        0,
        0,
        0,
    )
}

fn search(motions: &[char], visited: &mut [[bool; SIZE]], r: usize, c: usize, index: usize) -> i32 {
    if index == motions.len() {
        return 1;
    }
    if is_reaching_target_too_early(r, c)
        || is_horizontal_splitted(visited, r, c)
        || is_vertical_splitted(visited, r, c)
    {
        return 0;
    }

    let mut result = 0;
    visited[r][c] = true;

    for (motion, r_offset, c_offset) in OFFSETS {
        let adj_r = (r as i32) + r_offset;
        let adj_c = (c as i32) + c_offset;
        if (motions[index] == '?' || *motion == motions[index])
            && adj_r >= 0
            && adj_r < SIZE as i32
            && adj_c >= 0
            && adj_c < SIZE as i32
            && !visited[adj_r as usize][adj_c as usize]
        {
            result += search(motions, visited, adj_r as usize, adj_c as usize, index + 1);
        }
    }

    visited[r][c] = false;

    result
}

fn is_reaching_target_too_early(r: usize, c: usize) -> bool {
    r == SIZE - 1 && c == 0
}

fn is_horizontal_splitted(visited: &[[bool; SIZE]], r: usize, c: usize) -> bool {
    r != 0
        && !visited[r - 1][c]
        && r != SIZE - 1
        && !visited[r + 1][c]
        && (c == 0 || visited[r][c - 1])
        && (c == SIZE - 1 || visited[r][c + 1])
}

fn is_vertical_splitted(visited: &[[bool; SIZE]], r: usize, c: usize) -> bool {
    c != 0
        && !visited[r][c - 1]
        && c != SIZE - 1
        && !visited[r][c + 1]
        && (r == 0 || visited[r - 1][c])
        && (r == SIZE - 1 || visited[r + 1][c])
}
