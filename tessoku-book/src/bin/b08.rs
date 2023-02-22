fn main() {
    proconio::input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    // 点の数を座標ごとに合計
    let mut s = vec![vec![0; 1500]; 1500];
    for (x, y) in xy {
        s[y - 1][x - 1] += 1;
    }

    // 点の数をx軸方向に累積和を計算
    let mut acc = vec![vec![0; 1501]; 1501];
    for y in 1..=1500 {
        for x in 1..=1500 {
            acc[y][x] = acc[y][x - 1] + s[y - 1][x - 1];
        }
    }
    // 点の数をy軸方向に累積和を計算して、二次元累積和を計算
    for x in 1..=1500 {
        for y in 1..=1500 {
            acc[y][x] += acc[y - 1][x];
        }
    }
    // 指定された範囲の点の数を計算して出力
    for (a, b, c, d) in abcd {
        println!(
            "{}",
            acc[d][c] - acc[d][a - 1] - acc[b - 1][c] + acc[b - 1][a - 1]
        );
    }
}
