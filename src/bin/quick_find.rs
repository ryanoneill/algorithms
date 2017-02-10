use std::io;
use std::io::prelude::*;

const N: usize = 10000;

fn main() {
    let stdin = io::stdin();
    let mut id = [0; N];

    for item in 0 .. N {
        id[item] = item;
    }

    for line in stdin.lock().lines() {
        let value = line.unwrap();

        let items: Vec<u32> = value
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        let p = items[0];
        let q = items[1];

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
