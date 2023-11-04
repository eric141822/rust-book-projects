use std::{vec, collections::HashMap};

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut d = HashMap::new();
    for n in &v {
        let count = d.entry(*n).or_insert(0);
        *count += 1;
    }

    println!("Median: {}", get_median(&v));
    println!("Mode: {}", get_mode(&d));

    // for n in &v {
    //     println!("{}", n);
    // }

    // for (k, v) in &d {
    //     println!("{}: {}", k, v);
    // }
}

fn get_median(v: &Vec<i32>) -> i32 {
    let mut v = v.clone();
    v.sort();
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

