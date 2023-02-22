fn main() {
    proconio::input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }
    let result = a.iter().any(|v| *v == x);
    println!("{}", if result { "Yes" } else { "No" });
}
