use std::io;
use std::io::prelude::*;

use constants::N;
use connectivity::*;

pub fn run_quick_find() {
    run_quick_algorithm(quick_find);
}

pub fn run_quick_union() {
    run_quick_algorithm(quick_union);
}

pub fn run_weighted_quick_union() {
    run_weighted_algorithm(weighted_quick_union);
}

pub fn run_weighted_quick_union_with_path_compression() {
    run_weighted_algorithm(weighted_quick_union_with_path_compression);
}

fn run_weighted_algorithm<F>(process_connected_pair: F)
    where F: Fn(&mut [u32; N], &mut [u32; N], u32, u32) {

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
        process_connected_pair(&mut id, &mut sz, p, q);
    }

}

fn run_quick_algorithm<F>(process_connected_pair: F)
    where F: Fn(&mut [u32; N], u32, u32) {

    let stdin = io::stdin();
    let mut id: [u32; N] = [0; N];

    for item in 0 .. N {
        id[item] = item as u32;
    }

    for line in stdin.lock().lines() {
        let value = line.unwrap();
        let (p, q) = line_to_connected_pair(&value);
        process_connected_pair(&mut id, p, q);
    }
}

fn line_to_connected_pair(line: &str) -> (u32, u32) {
    let items: Vec<u32> = line
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    (items[0], items[1])
}
