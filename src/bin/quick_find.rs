extern crate algorithms;

use std::io;
use std::io::prelude::*;

use algorithms::*;

const N: usize = 10000;

fn main() {
    let stdin = io::stdin();
    let mut id: [u32; N] = [0; N];

    for item in 0 .. N {
        id[item] = item as u32;
    }

    for line in stdin.lock().lines() {
        let value = line.unwrap();
        let (p, q) = line_to_connected_pair(&value);

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
