fn main() {
    proconio::input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }
    println!("{}", a.binary_search(&x).unwrap() + 1);
}
