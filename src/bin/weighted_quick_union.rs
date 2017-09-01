extern crate algorithms;

use algorithms::{run_weighted_algorithm, N};

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

fn main() {
    run_weighted_algorithm(weighted_quick_union);
}
