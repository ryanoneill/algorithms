use std::io;
use std::io::prelude::*;

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

const N: usize = 10000;

fn quick_find(id: &mut [u32; N], p: u32, q: u32) {
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

fn quick_union(id: &mut [u32; N], p: u32, q: u32) {
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

fn weighted_quick_union(id: &mut [u32; N], sz: &mut [u32; N], p: u32, q: u32) {
    let mut i = p;
    while i != id[i as usize] {
        i = id[i as usize]
    }
    let mut j = q;
    while j != id[j as usize] {
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

fn weighted_quick_union_with_path_compression(id: &mut [u32; N], sz: &mut [u32; N], p: u32, q: u32) {
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
