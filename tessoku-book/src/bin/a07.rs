fn main() {
    proconio::input! {
        d: usize,
        n: usize,
    }
    let mut v = vec![0; d + 1];
    for _ in 0..n {
        proconio::input! {
            l: usize,
            r: usize,
        }
        v[l - 1] += 1;
        v[r] -= 1;
    }
    let mut sum = vec![0; d + 1];
    for i in 1..=d {
        sum[i] = sum[i - 1] + v[i - 1];
    }
    for i in 1..=d {
        println!("{}", sum[i]);
    }
}
