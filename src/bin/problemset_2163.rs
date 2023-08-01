use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let k = split.next().unwrap().parse().unwrap();

    println!("{}", solve(n, k));
}

fn solve(n: usize, k: usize) -> String {
    let size = (n as f64).sqrt().ceil() as usize;
    let mut blocks = Vec::new();
    for i in 0..n {
        if i % size == 0 {
            blocks.push(Vec::new());
        }

        blocks.last_mut().unwrap().push(i + 1);
    }

    let mut result = Vec::new();
    let mut block_index = 0;
    let mut inner_index = 0;
    for i in 0..n {
        let mut rest = k % (n - i);
        while rest > blocks[block_index].len() - inner_index - 1 {
            rest -= blocks[block_index].len() - inner_index;

            loop {
                block_index = (block_index + 1) % blocks.len();
                if !blocks[block_index].is_empty() {
                    break;
                }
            }
            inner_index = 0;
        }

        inner_index += rest;
        result.push(blocks[block_index].remove(inner_index));

        if i != n - 1 && inner_index == blocks[block_index].len() {
            loop {
                block_index = (block_index + 1) % blocks.len();
                if !blocks[block_index].is_empty() {
                    break;
                }
            }
            inner_index = 0;
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
