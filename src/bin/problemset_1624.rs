use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
};

const SIZE: usize = 8;

fn main() {
    let mut br = BufReader::new(stdin());

    let mut squares = [[0 as char; SIZE]; SIZE];
    for row in &mut squares {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        row.copy_from_slice(
            &split
                .next()
                .unwrap()
                .parse::<String>()
                .unwrap()
                .chars()
                .collect::<Vec<_>>(),
        );
    }

    println!("{}", solve(&squares));
}

fn solve(squares: &[[char; SIZE]]) -> i32 {
    search(squares, &mut (0..SIZE).collect::<Vec<_>>(), 0)
}

fn search(squares: &[[char; SIZE]], cols: &mut [usize], index: usize) -> i32 {
    if index == cols.len() {
        return if (0..SIZE).all(|r| squares[r][cols[r]] == '.')
            && (0..cols.len())
                .map(|r| r + cols[r])
                .collect::<HashSet<_>>()
                .len()
                == SIZE
            && (0..cols.len())
                .map(|r| (r as i32) - (cols[r] as i32))
                .collect::<HashSet<_>>()
                .len()
                == SIZE
        {
            1
        } else {
            0
        };
    }

    let mut result = 0;
    for i in index..SIZE {
        cols.swap(index, i);
        result += search(squares, cols, index + 1);
        cols.swap(index, i);
    }

    result
}
