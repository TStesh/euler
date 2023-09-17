use std::cmp::min;

pub fn ex_76(n: u8) -> u64 {
    let mut xs: Vec<u8> = (2..n - 1).collect();
    let mut ys: Vec<u8> = (2..n - 1).collect();
    let mut s = 2u64;
    let mut lw: u8;
    for k in 1..(n - 1) as usize {
        let mut new_xs: Vec<u8> = Vec::new();
        let mut new_ys: Vec<u8> = Vec::new();
        for (i, x) in xs.iter().enumerate() {
            for j in 1..=min(n - ys[i], *x) {
                lw = ys[i] + j;
                if lw < n {
                    new_xs.push(j);
                    new_ys.push(lw);
                } else {
                    s += 1
                }
            }
        }
        xs = new_xs;
        ys = new_ys;
    }
    s
}