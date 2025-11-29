use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let s: String = split.next().unwrap().parse().unwrap();

    println!("{}", solve(&s));
}

fn solve(s: &str) -> String {
    let mut counts = [0; 26];
    for c in s.as_bytes() {
        counts[(c - b'A') as usize] += 1;
    }

    if !is_possible(&counts) {
        return "-1".to_string();
    }

    let mut result = String::new();
    let mut prev_index = usize::MAX;
    for _ in 0..s.len() {
        let mut index = 0;
        loop {
            if index != prev_index && counts[index] != 0 {
                counts[index] -= 1;
                if is_possible(&counts) {
                    break;
                }

                counts[index] += 1;
            }

            index += 1;
        }

        result.push((index as u8 + b'A') as char);
        prev_index = index;
    }

    result
}

fn is_possible(counts: &[i32]) -> bool {
    let mut total = 0;
    let mut max_count = 0;

    for &count in counts {
        total += count;
        max_count = max_count.max(count);
    }

    max_count * 2 <= total + 1
}
