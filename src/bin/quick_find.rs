extern crate algorithms;

use algorithms::{run_quick_algorithm, N};

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

fn main() {
    run_quick_algorithm(quick_find);
}
