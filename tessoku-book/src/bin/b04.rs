fn main() {
    proconio::input! {
        mut n: i32,
    }
    let mut sum = 0;
    let mut base = 1;
    while 0 < n {
        let value = n % 10;
        sum += value * base;
        n /= 10;
        base *= 2;
    }
    println!("{}", sum);
}
