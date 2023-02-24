fn main() {
    proconio::input! {
        n: usize,
        h: [i32; n],
    }
    let mut c = vec![0i32; n];
    // 足場1のコストはh1
    c[0] = 0;
    // 足場2のコストは|h1 - h2|
    c[1] = (h[0] - h[1]).abs();
    // 足場3以降に移動するコストを計算
    for i in 2..n {
        // 足場i-1からのコストを計算
        let candidate1 = c[i - 1] + (h[i - 1] - h[i]).abs();
        // 足場i-2からのコストを計算
        let candidate2 = c[i - 2] + (h[i - 2] - h[i]).abs();
        // コストが小さい方を足場iまでの移動コストに設定
        c[i] = candidate1.min(candidate2);
    }
    println!("{}", c[n - 1]);
}
