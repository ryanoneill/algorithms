extern crate algorithms;

use algorithms::{run_quick_algorithm, N};

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

fn main() {
    run_quick_algorithm(quick_union);
}
