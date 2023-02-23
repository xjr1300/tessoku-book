fn main() {
    proconio::input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
    }
    a.sort();
    for _ in 0..q {
        proconio::input! {
            x: i32,
        }
        let mut index = a.binary_search(&x).unwrap_or_else(|index| index);
        while index > 1 && a[index - 1] == x {
            index -= 1;
        }
        println!("{}", index);
    }
}
