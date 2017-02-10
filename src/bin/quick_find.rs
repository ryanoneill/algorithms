use std::io;
use std::io::prelude::*;

const N: usize = 10000;

fn line_to_connected_pair(line: String) -> (u32, u32) {
    let items: Vec<u32> = line
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    (items[0], items[1])
}

fn main() {
    let stdin = io::stdin();
    let mut id = [0; N];

    for item in 0 .. N {
        id[item] = item;
    }

    for line in stdin.lock().lines() {
        let value = line.unwrap();
        let (p, q) = line_to_connected_pair(value);

        let t = id[p as usize];
        let u = id[q as usize];

        if t != u {
            for i in 0 .. N {
                if id[i] == t {
                    id[i] = u;
                }
            }
            println!("{} {}", p, q);
        }
    }
}
