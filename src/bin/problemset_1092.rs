use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();

    println!("{}", solve(n));
}

fn solve(n: i32) -> String {
    if n % 4 == 1 || n % 4 == 2 {
        return String::from("NO");
    }

    let mut set1 = Vec::new();
    let mut set2 = Vec::new();
    let begin = if n % 4 == 0 {
        1
    } else {
        set1.push(1);
        set1.push(2);
        set2.push(3);

        4
    };

    for i in (begin..=n).step_by(4) {
        set1.push(i);
        set2.push(i + 1);
        set2.push(i + 2);
        set1.push(i + 3);
    }

    format!(
        "YES\n{}\n{}\n{}\n{}",
        set1.len(),
        set1.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        set2.len(),
        set2.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
