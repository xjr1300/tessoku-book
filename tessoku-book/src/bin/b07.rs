fn main() {
    proconio::input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut v = vec![0i32; t + 1];
    for (l, r) in lr {
        v[l] += 1;
        v[r] -= 1;
    }
    let mut sum = vec![0i32; t + 1];
    for i in 1..=t {
        sum[i] = sum[i - 1] + v[i - 1];
    }
    for i in 1..=t {
        println!("{}", sum[i]);
    }
}
