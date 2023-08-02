use std::{
    cmp::Ordering,
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
        while let Some(&OccupiedOrder { index: i, .. }) = occupied.peek() {
            if b[i] >= a[index] {
                break;
            }

            available_rooms.push_back(allocated[occupied.pop().unwrap().index]);
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
        occupied.push(OccupiedOrder { index, b });
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

#[derive(PartialEq, Eq)]
struct OccupiedOrder<'a> {
    index: usize,
    b: &'a [i32],
}

impl Ord for OccupiedOrder<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.b[self.index].cmp(&other.b[other.index]).reverse()
    }
}

impl PartialOrd for OccupiedOrder<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
