#![allow(unused)]

use std::cmp::min;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use euler::{is_prime, segment_sieve};

fn ex_77_ways(n: u8) -> Vec<Vec<u8>> {
    let mut ws: Vec<Vec<u8>> = (1..n).map(|x| vec![x]).collect();
    let mut lw: u8;
    for _ in 0..n {
        let mut new_ws: Vec<Vec<u8>> = Vec::new();
        for w in ws.iter() {
            lw = w.iter().sum::<u8>();
            if lw < n {
                for i in 1..=min(n - lw, *w.last().unwrap()) {
                    let mut new_w = w.clone();
                    new_w.push(i);
                    new_ws.push(new_w);
                }
            } else {
                new_ws.push(w.clone());
            }
        }
        ws = new_ws;
    }
    ws
}

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

fn ex_78(n: u8) -> u64 {
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
    s + 1
}

fn p3(n: u64) -> u64 {
    let d = [0u64, 0, 0, 1, 1, 2];
    if n == 4 || n == 5 { return d[n as usize] }
    let (t, r) = (n / 6, n % 6);
    t * (3 * t + r) + d[r as usize]
}

fn pm(n: u64) -> HashMap<u64, Vec<u64>> {
    let mut hs: HashMap<u64, Vec<u64>> = HashMap::new();
    for m in 4..=n {
        hs.insert(m, vec![0u64; (m - 2) as usize]);
        hs.entry(m).and_modify(|v| {
            v[0] = m >> 1;
            v[1] = p3(m);
        });
    }
    for q in 4..=(n >> 1) {
        for m in (q << 1)..=n {
            let (mut s, mut r) = (m / q - 1, m % q);
            let ub = s as usize;
            let mut z = m - q;
            'l: for _ in 0..ub {
                for l in 0..=q - 3 {
                    if z < l + 3 { break 'l }
                    if z == l + 3 { s += 1; continue }
                    if z > (l << 1) + 4 {
                        let y = z - l - 2;
                        if y < 4 { s += y; continue }
                        let v = hs.get_key_value(&y).unwrap().1;
                        let vs = v.iter().fold(0u64, |a, x| a + *x);
                        s += vs + 3;
                    }
                    //println!("m = {m}, q={q}, z={z}, l={l}");
                    s += (hs.get_key_value(&(z as u64)).unwrap().1)[l as usize];
                }
                z -= q;
            }
            if r == 0 { r += 1 }
            hs.entry(m).and_modify(|v| v[(q - 2) as usize] = s + r);
        }
        for i in q << 1..n {
            if let Some(x) = hs.get_key_value(&i) {
                //println!("m={i}, q={q}, {:?}", x.1);
            }
        }
    }
    hs
}

pub fn check_perfect_number(num: i32) -> bool {
    if num < 6 || num & 1 == 1 { return false }
    let x = num.trailing_zeros();
    let y = num >> x;
    let z = (1 << (x + 1)) - 1;
    y == z && is_prime(z as u64)
}

fn main() {

    println!("{}", check_perfect_number(36));

    let dbg1 = false;
    let dbg2 = false;

    if dbg1 {
        let n = 30u64;
        let hs = pm(n);
        for i in 2..n {
            if let Some(x) = hs.get_key_value(&i) {
                println!("{i}: {:?}", x.1);
            }
        }
    }

    if dbg2 {
        const L: u8 = 11;
        let ws = ex_77_ways(L);
        let mut count = 0;
        let mut a = [0u64; L as usize];
        for w in ws {
            a[w[0] as usize] += 1;
            if w[0] == 5 {
                count += 1;
                println!("{count}: {:?}", w);
            }
        }
        println!("{:?}", a);
    }

    /*
    for i in 5u8..31 {
       let xs = ex_77_ways(i);
       let mut a = Vec::with_capacity(i as usize);
       unsafe { a.set_len(i as usize) }
       a.fill(0);
       for x in xs.iter() { a[x[0] as usize] += 1 }
       a.reverse();
       println!("{i}: {:?}", a);
    }
    */

    /*
    let start = std::time::Instant::now();
    let mut s = 1;
    let mut n = 100u8;
    while s % 1_000_000 != 0 {
        n += 1;
        s = ex_78(n);
        println!("{n}: {s}");
    }
    println!("Duration: {:?}", start.elapsed());
    println!("Result: {n}");
    */
}

