use std::collections::HashMap;
use euler::{segment_sieve, vec_sigma};

fn amicable_chain(n: u64, ubound: u64, sigmas: &[u64]) -> (Vec<u64>, u64) {
    let mut q = 1;
    let mut d = vec![n];
    let mut x = sigmas[n as usize] - n;
    loop {
        if x == 1 || x > ubound { return (d, 1) }
        if x == n { break }
        if d.contains(&x) {
            let mut d_new = vec![];
            for y in &d {
                if *y != x { d_new.push(*y); } else { return (d_new, 1) }
            }
        }
        d.push(x);
        x = sigmas[x as usize] - x;
        q += 1;
    }
    return (d, q)
}

pub fn problem_95(num: u64) -> u64 {
    let primes = segment_sieve(3 * (num >> 1), false);
    let sigmas = vec_sigma(3 * (num >> 1), &primes);
    let mut r = HashMap::new();
    for n in 6..num {
        if r.contains_key(&n) || primes.binary_search(&n).is_ok() { continue }
        let (d, q) = amicable_chain(n, num, &sigmas);
        for m in d { r.insert(m, q); }
    }
    let max_v = *r.values().into_iter().max().unwrap();
    let min_k = *r.keys()
        .into_iter()
        .filter(|k| *r.get_key_value(*k).unwrap().1 == max_v)
        .min().unwrap();
    min_k
}
