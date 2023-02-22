use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
    }
    let result = (0..n)
        .permutations(3)
        .any(|v| a[v[0]] + a[v[1]] + a[v[2]] == 1_000);
    println!("{}", if result { "Yes" } else { "No" });
}
