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
    let mut id: [u32; N] = [0; N];

    for item in 0 .. N {
        id[item] = item as u32;
    }

    for line in stdin.lock().lines() {
        let value = line.unwrap();
        let (p, q) = line_to_connected_pair(value);

        let mut i = p;
        while i != id[i as usize] {
            i = id[i as usize]
        }
        let mut j = q;
        while j != id[j as usize] {
            j = id[j as usize]
        }

        if i != j {
            id[i as usize] = j;
            println!("{} {}", p, q)
        }
    }
}