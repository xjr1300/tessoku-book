use itertools::Itertools;

fn combination_sum(a: &[i32], l: usize, r: usize) -> Vec<i32> {
    let mut v = vec![0];
    for i in 1..=r - l {
        for combi in (l..r).combinations(i) {
            let mut sum = 0;
            for i in combi {
                sum += a[i];
            }
            v.push(sum);
        }
    }

    v
}

fn main() {
    proconio::input! {
        n: usize,
        k: i32,
        a: [i32; n],
    }
    let m = n / 2;
    let mut p = combination_sum(&a, 0, m);
    p.sort();
    let mut q = combination_sum(&a, m, n);
    q.sort();
    let mut found = false;
    for n in p {
        if q.binary_search(&(k - n)).is_ok() {
            found = true;
            break;
        }
    }
    println!("{}", if found { "Yes" } else { "No" });
}
