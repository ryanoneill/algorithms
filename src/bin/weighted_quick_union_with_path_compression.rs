extern crate algorithms;

use std::io;
use std::io::prelude::*;

use algorithms::*;

const N: usize = 10000;

fn main() {
    let stdin = io::stdin();

    let mut id: [u32; N] = [0; N];
    let mut sz: [u32; N] = [0; N];
    for item in 0 .. N {
        id[item] = item as u32;
        sz[item] = 1;
    }

    for line in stdin.lock().lines() {
        let value = line.unwrap();
        let (p, q) = line_to_connected_pair(&value);

        let mut i = p;
        while i != id[i as usize] {
            id[i as usize] = id[id[i as usize] as usize];
            i = id[i as usize];
        }
        let mut j = q;
        while j != id[j as usize] {
            id[j as usize] = id[id[j as usize] as usize];
            j = id[j as usize]
        }

        if i != j {
            if sz[i as usize] < sz[j as usize] {
                id[i as usize] = j;
                sz[j as usize] += sz[i as usize]
            } else {
                id[j as usize] = i;
                sz[i as usize] += sz[j as usize]
            }
            println!("{} {}", p, q)
        }
    }
}
