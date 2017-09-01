use constants::N;

pub fn quick_find(id: &mut [u32; N], p: u32, q: u32) {
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

pub fn quick_union(id: &mut [u32; N], p: u32, q: u32) {
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

pub fn weighted_quick_union(id: &mut [u32; N], sz: &mut [u32; N], p: u32, q: u32) {
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

pub fn weighted_quick_union_with_path_compression(id: &mut [u32; N], sz: &mut [u32; N], p: u32, q: u32) {
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

