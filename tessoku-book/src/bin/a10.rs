fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        d: usize,
        lr: [(usize, usize); d],
    }
    let mut forward = vec![0; n + 2];
    for i in 1..=n {
        if a[i - 1] < forward[i - 1] {
            forward[i] = forward[i - 1];
        } else {
            forward[i] = a[i - 1];
        }
    }
    let mut backward = vec![0; n + 2];
    for i in (1..=n).rev() {
        if a[i - 1] < backward[i + 1] {
            backward[i] = backward[i + 1];
        } else {
            backward[i] = a[i - 1];
        }
    }
    for (l, r) in lr {
        println!("{}", forward[l - 1].max(backward[r + 1]));
    }
}
