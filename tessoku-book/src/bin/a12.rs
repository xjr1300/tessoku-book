fn num_of_printed(a: &[u64], seconds: u64) -> u64 {
    let mut num = 0;
    for x in a {
        num += seconds / x;
    }

    num
}

fn main() {
    proconio::input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let mut l = 1;
    let mut r = 10u64.pow(9);
    while l < r {
        let i = (r + l) / 2;
        let num = num_of_printed(&a, i);
        if k <= num {
            r = i;
        } else {
            l = i + 1;
        }
    }
    println!("{}", l);
}
