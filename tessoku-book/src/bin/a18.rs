fn main() {
    proconio::input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    // カードを0枚選択するとき、合計値は0
    dp[0][0] = true;
    // iは選択するカードの枚数
    for i in 1..=n {
        // 合計値がjになるようなカードを選択できるか確認
        // a[i-1]は選択するカードの値
        // 方法A: カードi-1の時点で合計がjであり、カードiを選ばない
        // 方法B: カードi-1の時点で合計がj-Aiであり、カードiを選ぶ
        for j in 0..=s {
            if j < a[i - 1] {
                // 合計値jが今回選択するカードより小さい場合、今回カードを選択しない
                dp[i][j] = dp[i - 1][j];
            } else {
                // カードを選択しないか、カードを選択したとき合計値がjになるか
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - a[i - 1]]
            }
        }
    }
    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}
