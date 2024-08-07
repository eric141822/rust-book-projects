use std::{vec, collections::HashMap};

fn main() {
    let v = vec![1, 2, 3, 4, 5, 5];
    let mut d = HashMap::new();
    for n in &v {
        // deref the n: &i32 with *n.
        let count = d.entry(*n).or_insert(0);
        *count += 1; // increase the value of the key n.
    }

    println!("Median: {}", get_median(&v));
    println!("Mode: {}", get_mode(&d));


    print!("vec: ");
    for n in &v {
        print!("{} ", n);
    }
    println!();

    print!("dict: ");
    for (k, v) in &d {
        print!("{}: {}\t", k, v);
    }
    println!();
}

fn get_median(v: &[i32]) -> i32 {
    let mut v = v.to_vec();
    v.sort_unstable(); // order of equal elements is not guaranteed, but doesn't matter.
    if v.len() % 2 == 0 {
        return (v[v.len() / 2] + v[v.len() / 2 - 1]) / 2;
    }
    v[v.len() / 2]
}

fn get_mode(d: &HashMap<i32, i32>) -> i32 {
    let mut max = -1;
    let mut mode = 0;
    for (k, v) in d {
        if *v > max {
            max = *v;
            mode = *k;
        }
    }
    mode
}

