fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        x: [[i32; w]; h],
        q: usize,
        a: [(usize, usize, usize, usize); q],
    }
    let mut acc = vec![vec![0; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            acc[i][j] = acc[i][j - 1] + x[i - 1][j - 1];
        }
    }
    for j in 1..=w {
        for i in 1..=h {
            acc[i][j] += acc[i - 1][j];
        }
    }
    for (a, b, c, d) in a {
        println!(
            "{}",
            acc[c][d] + acc[a - 1][b - 1] - acc[c][b - 1] - acc[a - 1][d]
        )
    }
}
