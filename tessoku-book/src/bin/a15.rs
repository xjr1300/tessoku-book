use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
    }
    let mut b = a.clone();
    b.sort();
    b.dedup();
    let mut m = HashMap::new();
    for (i, v) in b.iter().enumerate() {
        m.insert(v, i + 1);
    }
    for n in a.iter() {
        print!("{} ", m.get(&n).unwrap());
    }
    println!();
}
