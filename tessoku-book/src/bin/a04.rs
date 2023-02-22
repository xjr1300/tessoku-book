fn main() {
    proconio::input! {
        mut n: i32,
    }
    let mut bin = [0; 10];
    let mut i = 9i32;
    while 0 < n && 0 <= i {
        bin[i as usize] = n % 2;
        i -= 1;
        n /= 2;
    }
    for i in bin.iter() {
        print!("{}", i);
    }
    println!();
}
