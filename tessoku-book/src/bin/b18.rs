use std::process::exit;

fn main() {
    proconio::input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    // カードを0枚選択
    dp[0][0] = true;
    // i枚カードを選択
    for i in 1..=n {
        // 選択するカードの値はa[i-1]
        // カードを選択したとき、合計値をjにできるか
        for j in 0..=s {
            if j < a[i - 1] {
                // a[i-1]を選択しない場合
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - a[i - 1]];
            }
        }
    }
    if !dp[n][s] {
        println!("-1");
        exit(0);
    }
    // iに進むパスを検索
    let mut r = s;
    let mut i = n;
    let mut indices = vec![];
    while 0 < r {
        if !dp[i - 1][r] {
            r -= a[i - 1];
            indices.push(i);
        }
        i -= 1;
    }
    println!("{}", indices.len());
    for index in indices.iter().rev() {
        print!("{} ", index);
    }
    println!();
}
