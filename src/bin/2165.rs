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
    let mut moves = Vec::new();
    search(&mut moves, n, 1, 3, 2);

    format!(
        "{}\n{}",
        moves.len(),
        moves
            .iter()
            .map(|m| format!("{} {}", m.from_stack, m.to_stack))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn search(moves: &mut Vec<Move>, disk_num: i32, from_stack: i32, to_stack: i32, helper_stack: i32) {
    if disk_num != 1 {
        search(moves, disk_num - 1, from_stack, helper_stack, to_stack);
    }

    moves.push(Move {
        from_stack,
        to_stack,
    });

    if disk_num != 1 {
        search(moves, disk_num - 1, helper_stack, to_stack, from_stack);
    }
}

struct Move {
    from_stack: i32,
    to_stack: i32,
}
