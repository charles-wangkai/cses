use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", solve(&a, &b));
}

fn solve(a: &[i32], b: &[i32]) -> String {
    let mut sorted_indices: Vec<_> = (0..a.len()).collect();
    sorted_indices.sort_by_key(|&i| a[i]);

    let mut room_count = 0;
    let mut allocated = vec![0; a.len()];
    let mut available_rooms = VecDeque::new();
    let mut occupied = BinaryHeap::new();
    for index in sorted_indices {
        while let Some(&(_, i)) = occupied.peek() {
            if b[i] >= a[index] {
                break;
            }

            available_rooms.push_back(allocated[occupied.pop().unwrap().1]);
        }

        match available_rooms.pop_front() {
            Some(room) => {
                allocated[index] = room;
            }
            None => {
                room_count += 1;
                allocated[index] = room_count;
            }
        }
        occupied.push((Reverse(b[index]), index));
    }

    format!(
        "{}\n{}",
        room_count,
        allocated
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
