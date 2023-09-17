use std::cmp::min;

fn ex_77(m: u16) -> u64 {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43,
        47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let mut n = 10u8;
    loop {
        let mut xs: Vec<u8> = (2..n - 1).collect();
        let mut ys: Vec<u8> = (2..n - 1).collect();
        let mut s = 0u64;
        let mut lw: u8;
        for k in 1..(n - 1) as usize {
            let mut new_xs: Vec<u8> = Vec::new();
            let mut new_ys: Vec<u8> = Vec::new();
            for (i, x) in xs.iter().enumerate() {
                if primes.binary_search(&(*x as u64)).is_err() { continue }
                for j in 2..=min(n - ys[i], *x) {
                    if primes.binary_search(&(j as u64)).is_err() { continue }
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
        if s > m as u64 { return n as u64 }
        n += 1;
    }
}

pub fn problem_77() -> u64 {
    ex_77(5_000)
}