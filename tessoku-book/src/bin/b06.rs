fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        q: usize,
    }
    let mut sum = vec![0; n + 1];
    for i in 1..=n {
        sum[i] = sum[i - 1] + a[i - 1];
    }
    for _ in 0..q {
        proconio::input! {
            l:usize,
            r: usize,
        }
        let num_of_wins = sum[r] - sum[l - 1];
        let num_of_tries = (r - l + 1) as i32;
        let threshold = num_of_tries / 2;
        if num_of_wins > threshold {
            println!("win");
        } else if num_of_wins == threshold && num_of_tries % 2 == 0 {
            println!("draw");
        } else {
            println!("lose");
        }
    }
}
