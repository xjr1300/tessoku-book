fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        a: [i32; n],
        lr: [(usize, usize); q],
    }
    let mut sum = vec![0; n + 1];
    for i in 1..=n {
        sum[i] = sum[i - 1] + a[i - 1];
    }
    for (l, r) in lr {
        println!("{}", sum[r] - sum[l - 1]);
    }
}
