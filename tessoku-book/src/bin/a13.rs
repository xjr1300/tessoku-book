fn main() {
    proconio::input! {
        n: usize,
        k: i32,
        a: [i32; n],
    }
    let mut r = vec![n - 1; n];
    let mut i = 0;
    let mut j = 0;
    while i < n {
        r[i] = j;
        while j < n && a[j] - a[i] <= k {
            j += 1;
            r[i] = j;
        }
        i += 1;
    }
    let mut num = 0;
    for i in 0..n {
        num += r[i] - (i + 1);
    }
    println!("{}", num);
}
