fn main() {
    proconio::input! {
        n: f64,
    }
    let mut l = 0.0;
    let mut r = n;
    let mut v = 0.0;
    let mut m = -1.0;
    while (v - n).abs() > 1e-5 {
        m = (r + l) / 2.0;
        v = m * m * m + m;
        if v < n {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{:.6}", m);
}
