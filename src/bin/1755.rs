use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let s: String = split.next().unwrap().parse().unwrap();

    println!("{}", solve(&s));
}

fn solve(s: &str) -> String {
    let mut letter_to_count = HashMap::new();
    for letter in s.chars() {
        letter_to_count
            .entry(letter)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let odd_count_letters: Vec<_> = letter_to_count
        .keys()
        .copied()
        .filter(|letter| letter_to_count.get(letter).unwrap() % 2 == 1)
        .collect();
    if odd_count_letters.len() > 1 {
        return String::from("NO SOLUTION");
    }

    let half = letter_to_count
        .iter()
        .map(|(letter, count)| letter.to_string().repeat(count / 2))
        .collect::<String>();

    format!(
        "{}{}{}",
        half,
        odd_count_letters.iter().collect::<String>(),
        half.chars().rev().collect::<String>()
    )
}
