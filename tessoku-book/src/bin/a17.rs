fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n-1],
        b: [i32; n -2],
    }
    // 動的計画法で各部屋への最短移動時間を計算
    let mut times = vec![0; n];
    // 部屋1は0分で到着
    // 部屋2はa[0]分で到着
    times[1] = a[0];
    // 部屋3以後の部屋へ到着する最短移動時間を計算
    for i in 2..n {
        // 部屋i-1から部屋iに移動する時間
        let candidate1 = times[i - 1] + a[i - 1];
        // 部屋i-2から部屋iに移動する時間
        let candidate2 = times[i - 2] + b[i - 2];
        // 上記移動時間のうち最短移動時間を選択
        times[i] = candidate1.min(candidate2);
    }
    let mut rooms = vec![n - 1];
    let mut i = n - 1;
    loop {
        // 部屋i-1から部屋iに移動する時間
        let candidate1 = times[i - 1] + a[i - 1];
        if candidate1 == times[i] {
            rooms.push(i - 1);
            i -= 1;
        } else {
            rooms.push(i - 2);
            i -= 2;
        }
        if i == 0 {
            break;
        }
    }
    println!("{}", rooms.len());
    for n in rooms.iter().rev() {
        print!("{} ", n + 1);
    }
    println!();
}
