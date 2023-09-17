use std::collections::HashMap;

// Умный расчет
pub fn fast_routes_amount(num: u64) -> u64 {
    let n = num + 1;
    let n2 = n << 1;
    let mut d = HashMap::new();
    // a = x, b + c = y
    let mut end = ((n as f64) * (1. + 5f64.sqrt()) / 2.).sqrt() as u64;
    for u in 2..=end {
        let u2 = u * u;
        let fu = u & 1;
        let mut uv = u;
        let mut v = 1u64;
        while v < u && uv <= n {
            let fv = v & 1;
            let mut x = u2 - v * v;
            let mut y = uv << 1;
            if x <= n && ((x < y && 1 + x > uv) || (x > y)) {
                if fu * fv == 1 {
                    x >>= 1;
                    y >>= 1;
                }
                let mut p = x;
                let mut q = y;
                while p < n && q <= n2 {
                    let mut r = if p < q { 1 + p - (q >> 1) } else { q >> 1 };
                    if p < q && q < (p << 1) && p & 1 == 0 && q & 1 == 1 { r -= 1; }
                    d.insert((p, q), r);
                    p += x;
                    q += y;
                }
            }
            v += 1;
            uv = u * v;
        }
    }
    // b + c = x, a = y
    end = 1 + ((n2 + 1) as f64).sqrt() as u64;
    for u in 2..=end {
        let u2 = u * u;
        let fu = u & 1;
        let mut uv = u;
        let mut v = 1u64;
        while v < u && uv <= n {
            let fv = v & 1;
            let x = u2 - v * v;
            let y = uv << 1;
            let y2 = y << 1;
            if (fu + fv) == 1 && y < n && x <= y2 && !d.contains_key(&(y, x)) {
                let mut r = if y < x { 1 + y - (x >> 1) } else { x >> 1 };
                if y < x && x < y2 && y & 1 == 0 && x & 1 == 1 { r -= 1; }
                d.insert((y, x), r);
            }
            v += 1;
            uv = u * v;
        }
    }
    // routes amount
    d.values().sum()
}


// Расчет в лоб
pub fn routes_amount(num: u64) -> u64 {
    let mut l: u64;
    let mut q: u64;
    let mut ls: u64;
    let mut a2: u64;
    let mut r = 0u64;
    let n = num + 1;
    for a in 2..=n {
        a2 = a * a;
        for b in 1..=a {
            for c in 1..=b {
                q = b + c;
                l = a2 + q * q;
                ls = (l as f64).sqrt() as u64;
                if ls * ls == l { r += 1; }
            }
        }
    }
    r
}
