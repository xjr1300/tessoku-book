fn main() {
    proconio::input! {
        n: i32,
        k: i32,
    }
    let mut counter = 0;
    // 赤色のカードを記入
    for r in 1..=n {
        // 青色のカードを記入
        for b in 1..=n {
            // 白色のカードを計算
            let w = k - r - b;
            counter += if 1 <= w && w <= n { 1 } else { 0 };
        }
    }
    println!("{}", counter);
}
