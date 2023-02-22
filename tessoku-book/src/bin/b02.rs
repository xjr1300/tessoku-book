fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    let result = (a..=b).any(|n| 100 % n == 0);
    println!("{}", if result { "Yes" } else { "No" });
}
