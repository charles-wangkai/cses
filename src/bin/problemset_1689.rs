// https://en.wikipedia.org/wiki/Knight%27s_tour#Warnsdorff's_rule

use std::io::{stdin, BufRead, BufReader};

const SIZE: usize = 8;

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let x = split.next().unwrap().parse().unwrap();
    let y = split.next().unwrap().parse().unwrap();

    println!("{}", solve(x, y));
}

fn solve(x: usize, y: usize) -> String {
    let mut moves = vec![vec![0; SIZE]; SIZE];
    search(&mut moves, 1, y - 1, x - 1);

    moves
        .iter()
        .map(|line| {
            line.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn search(moves: &mut [Vec<usize>], step: usize, r: usize, c: usize) -> bool {
    moves[r][c] = step;

    if step == 64 {
        return true;
    }

    let next_points = find_next_points(moves, r, c);
    if !next_points.is_empty() {
        let onward_move_nums: Vec<_> = next_points
            .iter()
            .map(|next_point| compute_onward_move_nums(moves, next_point))
            .collect();
        let min_onward_move_num = onward_move_nums.iter().min().copied().unwrap();
        for i in 0..onward_move_nums.len() {
            if onward_move_nums[i] == min_onward_move_num
                && search(moves, step + 1, next_points[i].r, next_points[i].c)
            {
                return true;
            }
        }
    }

    moves[r][c] = 0;

    false
}

fn compute_onward_move_nums(moves: &[Vec<usize>], next_point: &Point) -> usize {
    find_next_points(moves, next_point.r, next_point.c).len()
}

fn find_next_points(moves: &[Vec<usize>], r: usize, c: usize) -> Vec<Point> {
    let mut result = Vec::new();
    for dr in -2i32..=2 {
        for dc in -2..=2 {
            if (dr * dc).abs() == 2 {
                let next_r = (r as i32) + dr;
                let next_c = (c as i32) + dc;
                if next_r >= 0
                    && next_r < SIZE as i32
                    && next_c >= 0
                    && next_c < SIZE as i32
                    && moves[next_r as usize][next_c as usize] == 0
                {
                    result.push(Point {
                        r: next_r as usize,
                        c: next_c as usize,
                    })
                }
            }
        }
    }

    result
}

struct Point {
    r: usize,
    c: usize,
}
