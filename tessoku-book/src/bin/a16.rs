fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n-1],
        b: [i32; n-2],
    }
    // i番目の部屋に移動する最小時間を格納するベクタ
    let mut t = vec![0; n];
    // 1番目の部屋には0分で到着
    t[0] = 0;
    // 2番目の部屋にはa[0]分で到着
    t[1] = a[0];
    // 3番目以降の部屋に到着する分数を計算
    for i in 2..n {
        // i-2番目の部屋からb[i-2]分で到着
        let c1 = t[i - 2] + b[i - 2];
        // i-1番目の部屋からa[i-1]分で到着
        let c2 = t[i - 1] + a[i - 1];
        // 分数が小さい方を当該部屋に到着する変数に設定
        t[i] = c1.min(c2);
    }
    println!("{}", t[n - 1]);
}
