fn main() {
    proconio::input! {
        n: usize,
        h: [i32; n],
    }
    let mut costs = vec![0; n];
    // 足場2のコストを計算
    costs[1] = (h[0] - h[1]).abs();
    // 足場3から足場nまでのコストを計算
    for i in 2..n {
        // 足場i-1から足場iまでのコストを計算
        let cost1 = costs[i - 1] + (h[i - 1] - h[i]).abs();
        // 足場i-2から足場iまでのコストを計算
        let cost2 = costs[i - 2] + (h[i - 2] - h[i]).abs();
        // 足場iのコストを設定
        costs[i] = cost1.min(cost2);
    }
    // 足場nから足場1までの移動経路を計算
    let mut scaffolds = Vec::with_capacity(n);
    scaffolds.push(n - 1);
    let mut i = n - 1;
    if i == 0 {
        scaffolds.push(0);
    } else {
        loop {
            // 足場i-1から足場iに異動したときのコストを計算
            let cost = costs[i - 1] + (h[i - 1] - h[i]).abs();
            if cost == costs[i] {
                i -= 1;
            } else {
                i -= 2;
            }
            scaffolds.push(i);
            if i == 0 {
                break;
            }
        }
    }
    println!("{}", scaffolds.len());
    for s in scaffolds.iter().rev() {
        print!("{} ", s + 1)
    }
    println!();
}
