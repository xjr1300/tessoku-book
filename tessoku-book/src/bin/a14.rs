fn gen_all_sum(x: &[i32], y: &[i32]) -> Vec<i32> {
    let mut v = vec![];
    for i in 0..x.len() {
        for j in 0..y.len() {
            v.push(x[i] + y[j]);
        }
    }

    v
}

fn main() {
    proconio::input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
        c: [i32; n],
        d: [i32; n],
    }

    let mut p = gen_all_sum(&a, &b);
    p.sort();
    let mut q = gen_all_sum(&c, &d);
    q.sort();
    let mut found = false;
    for x in p {
        let result = q.binary_search(&(k - x));
        if result.is_ok() {
            found = true;
            break;
        }
    }
    println!("{}", if found { "Yes" } else { "No" });
}
