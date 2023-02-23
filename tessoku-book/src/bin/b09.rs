fn main() {
    proconio::input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let width = 1500;
    let height = 1500;
    let mut v = vec![vec![0; width + 1]; height + 1];
    for (a, b, c, d) in abcd {
        v[b][a] += 1;
        v[d][a] -= 1;
        v[b][c] -= 1;
        v[d][c] += 1;
    }
    for y in 0..height + 1 {
        for x in 1..width + 1 {
            v[y][x] += v[y][x - 1];
        }
    }
    for x in 0..width + 1 {
        for y in 1..height + 1 {
            v[y][x] += v[y - 1][x];
        }
    }
    let mut area = 0;
    for y in 0..height + 1 {
        for x in 0..width + 1 {
            if v[y][x] > 0 {
                area += 1;
            }
        }
    }
    println!("{}", area);
}
