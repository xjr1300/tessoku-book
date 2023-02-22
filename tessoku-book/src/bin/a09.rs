fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    let mut v = vec![vec![0; w + 1]; h + 1];
    for (a, b, c, d) in abcd {
        v[a - 1][b - 1] += 1;
        v[c][d] += 1;
        v[c][b - 1] -= 1;
        v[a - 1][d] -= 1;
    }
    // 二次元累積和を計算
    let mut acc = vec![vec![0; w + 2]; h + 2];
    // ｘ軸方向に累積和を計算
    for y in 1..=h {
        for x in 1..=w {
            acc[y][x] = acc[y][x - 1] + v[y - 1][x - 1];
        }
    }
    // y軸方向に累積和を計算
    for x in 1..=w {
        for y in 1..=h {
            acc[y][x] += acc[y - 1][x];
        }
    }
    // 積雪深を出力
    for y in 1..=h {
        for x in 1..=w {
            print!("{} ", acc[y][x]);
        }
        println!();
    }
}
