fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    // 累積和を計算
    let mut acc = vec![0; n + 1];
    for i in 1..=n {
        acc[i] = acc[i - 1] + a[i - 1];
    }
    // 尺取法で計算
    let mut r = vec![n; n + 1];
    for i in 1..=n {
        r[i] = if i == 1 { 0 } else { r[i - 1] };
        while r[i] < n && acc[r[i] + 1] - acc[i - 1] <= k {
            r[i] += 1;
        }
    }
    // 何通り買い方があるか計算して出力
    let mut num = 0;
    for i in 1..=n {
        num += r[i] + 1 - i;
    }
    println!("{:?}", num);
}
