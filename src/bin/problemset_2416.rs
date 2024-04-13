// https://usaco.guide/problems/cses-2416-increasing-array-queries/solution

use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let q = split.next().unwrap().parse().unwrap();
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let mut x = Vec::new();
    for _ in 0..n {
        x.push(split.next().unwrap().parse().unwrap());
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&x, &a, &b));
}

fn solve(x: &[i32], a: &[usize], b: &[usize]) -> String {
    let mut prefix_sums = vec![0; x.len()];
    for i in 0..prefix_sums.len() {
        prefix_sums[i] = (if i == 0 { 0 } else { prefix_sums[i - 1] }) + (x[i] as i64);
    }

    let mut begin_index_to_query_indices = HashMap::new();
    for (i, &ai) in a.iter().enumerate() {
        begin_index_to_query_indices
            .entry(ai - 1)
            .or_insert_with(Vec::new);

        begin_index_to_query_indices
            .get_mut(&(ai - 1))
            .unwrap()
            .push(i);
    }

    let mut result = vec![0; a.len()];
    let mut segments: Vec<Segment> = Vec::new();
    for begin_index in (0..x.len()).rev() {
        let mut end_index = begin_index;
        while !segments.is_empty() && x[begin_index] >= x[segments.last().unwrap().begin_index] {
            end_index = segments.pop().unwrap().end_index;
        }

        segments.push(Segment {
            begin_index,
            end_index,
            prefix_sum: ((x[begin_index] as i64) * ((end_index - begin_index + 1) as i64)
                - compute_range_sum(&prefix_sums, begin_index, end_index))
                + (if segments.is_empty() {
                    0
                } else {
                    segments.last().unwrap().prefix_sum
                }),
        });

        if let Some(query_indices) = begin_index_to_query_indices.get(&begin_index) {
            for &query_index in query_indices {
                let end_index = b[query_index] - 1;
                let segment_index = find_segment_index(&segments, end_index);

                result[query_index] = segments.last().unwrap().prefix_sum
                    - segments[segment_index].prefix_sum
                    + ((x[segments[segment_index].begin_index] as i64)
                        * ((end_index - segments[segment_index].begin_index + 1) as i64)
                        - compute_range_sum(
                            &prefix_sums,
                            segments[segment_index].begin_index,
                            end_index,
                        ));
            }
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_segment_index(segments: &[Segment], end_index: usize) -> usize {
    let mut result = 0;
    let mut lower = 0;
    let mut upper = (segments.len() as i32) - 1;
    while lower <= upper {
        let middle = (lower + upper) / 2;
        if segments[middle as usize].begin_index > end_index {
            lower = middle + 1;
        } else {
            result = middle as usize;
            upper = middle - 1;
        }
    }

    result
}

fn compute_range_sum(prefix_sums: &[i64], begin_index: usize, end_index: usize) -> i64 {
    prefix_sums[end_index]
        - (if begin_index == 0 {
            0
        } else {
            prefix_sums[begin_index - 1]
        })
}

struct Segment {
    begin_index: usize,
    end_index: usize,
    prefix_sum: i64,
}
