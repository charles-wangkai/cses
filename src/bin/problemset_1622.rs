use std::{
    collections::BTreeSet,
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
    let mut strings = BTreeSet::new();
    search(&mut strings, &mut s.chars().collect(), 0);

    format!(
        "{}\n{}",
        strings.len(),
        strings.into_iter().collect::<Vec<_>>().join("\n")
    )
}

fn search(strings: &mut BTreeSet<String>, letters: &mut Vec<char>, index: usize) {
    if index == letters.len() {
        strings.insert(letters.iter().collect::<String>());

        return;
    }

    for i in index..letters.len() {
        letters.swap(index, i);
        search(strings, letters, index + 1);
        letters.swap(index, i);
    }
}
