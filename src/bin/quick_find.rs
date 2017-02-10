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
        let test = line.unwrap();
        let items: Vec<&str> = test.split(' ').collect();

        let p: usize = items[0].parse().unwrap();
        let q: usize = items[1].parse().unwrap();

        let t = id[p];
        let u = id[q];

        if t == u {
            continue;
        } else {
            for i in 0 .. N {
                if id[i] == t {
                    id[i] = u;
                }
            }
            println!(" {} {}", p, q);
        }
    }
}
