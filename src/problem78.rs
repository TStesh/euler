use std::collections::HashMap;

const N: usize = 100_000;
const D: u32 = 1_000_000;

pub struct Cash {
    h: HashMap<(u32, u32), u32>,
    w: [u32; N],
    ws: [u32; N]
}

impl Cash {
    fn new() -> Self {
        Self { h: HashMap::new(), w: [0; N], ws: [0; N] }
    }
    fn init(&mut self, n: u32) {
        self.h.insert((2, 1), 1);
        self.h.insert((2, 2), 1);
        for m in 3..=n {
            self.h.insert((m, 2), m >> 1);
            self.h.insert((m, m), 1);
            self.h.insert((m, m - 1), 1);
            let a = m / 3;
            self.h.insert((m, 3), ((a * (m + m % 3) + (a & 1)) >> 2) % D);
        }
        self.w[1] = 1; self.w[2] = 2; self.w[3] = 3;
        self.ws[1] = 3; self.ws[2] = 5; self.ws[3] = 8;
    }
    // p(n, m) = p(n - m, m) + p(n - 1, m - 1)
    fn pn(&mut self, n: u32, m: u32) {
        let nm = n - m;
        let n1 = n - 1;
        let m1 = m - 1;
        //if nm >= m + 2 && self.h.contains_key(&(nm, m)) && self.h.contains_key(&(n1, m1)) {
        if nm >= m + 2 {
            let a  = *self.h.get_key_value(&(nm, m)).unwrap().1;
            let b = *self.h.get_key_value(&(n1, m1)).unwrap().1;
            self.h.insert((n, m), (a + b) % D);
        } else {
            self.pm(n, m)
        };
    }
    // p(n, m) = ∑ p(n - km + (m - 1), m - 1) [1 ≤ k ≤ [n / m]]
    fn pm(&mut self, n: u32, m: u32) {
        let m1 = m - 1;
        let mut res = 0;
        let mut nc = n - 1;
        for _ in 1..n / m {
            res += *self.h.get_key_value(&(nc, m1)).unwrap().1;
            res %= D;
            nc -= m;
        }
        res += *self.h.get_key_value(&(nc, m1)).unwrap().1;
        res %= D;
        self.h.insert((n, m), res);
    }
    // ways(n) = 3 + ∑ p(n, m) [2 ≤ m ≤ [n / 2]] + ∑ ways(m) [2 ≤ m ≤ n - [n / 2] - 1]
    fn wn(&mut self, n: u32) {
        let q = n >> 1;
        let b = n & 1;
        let mut res = self.ws[(q + b - 1) as usize];
        for m in 2..=q {
            res += *self.h.get_key_value(&(n, m)).unwrap().1;
            res %= D;
        }
        let nu = n as usize;
        self.w[nu] = res;
        self.ws[nu] = (self.ws[nu - 1] + res) % D;
    }
}

pub fn problem_78(n: u32) -> u32 {
    let mut cash = Cash::new();
    cash.init(n);
    let q = n >> 1;
    for a in 4..=q {
        for b in a + 2..=n { cash.pn(b, a); }
        cash.wn(a);
        if cash.w[a as usize] == 0 { return a }
    }
    for a in q + 1..=n {
        cash.wn(a);
        if cash.w[a as usize] == 0 { return a }
    }
    //println!("{}", *cash.h.get_key_value(&(30, 8)).unwrap().1);
    //for x in 4..31 {
    //    println!("ways({x}) = {}", cash.w[x]);
    //}
    0
}
