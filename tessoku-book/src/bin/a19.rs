fn main() {
    proconio::input! {
        n: usize,
        w: i32,
        wv: [(usize, i64); n],
    }
    let mut dp = vec![vec![std::i64::MIN; w as usize + 1]; n + 1];
    // 何も選択しないときの価値を設定
    dp[0][0] = 0;
    // 品物1以降を選択するか、選択しないかでdpを設定
    for i in 1..=n {
        // 0からwまでループ
        for j in 0usize..=w as usize {
            if j < wv[i - 1].0 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - wv[i - 1].0] + wv[i - 1].1);
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}
