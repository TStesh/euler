fn partition(num: i32) -> Vec<Vec<i32>> {
    let mut r = vec![];
    for a in 1..7 {
        for b in a + 1..8 {
            for c in b + 1..9 {
                for d in c + 1..10 {
                    let mut v = vec![a, b, c, d];
                    let e = num - a - b - c - d;
                    if e <= d || e > 10 || v.contains(&e) { continue }
                    v.push(e);
                    r.push(v);
                }
            }
        }
    }
    r
}

pub fn problem68() -> String {
    let m = [
        [-1, -1, 1, 1, 1],
        [1, -1, -1, 1, 1],
        [1, 1, -1, -1, 1],
        [1, 1, 1, -1, -1],
        [-1, 1, 1, 1, -1]
    ];

    let w = [
        [5, 1, 0],
        [6, 2, 1],
        [7, 3, 2],
        [8, 4, 3],
        [9, 0, 4]
    ];

    let mut res = vec![];

    for lvl in 14..20 {
        let cf = 55 - 3 * lvl;
        for x in partition(5 * (lvl - 11)) {
            // println!("x={x:?}");
            let mut rs = vec![];
            'l: for p in euler::permutations(&x) {
                // println!("p={p:?}");
                let mut v: Vec<_> = p.iter().map(|y| *y << 1).collect();
                for i in 0..5 {
                    let mut f = cf;
                    for j in 0..5 { f += m[i][j] * p[j]; }
                    if f > 20 || f & 1 != 0 || v.contains(&f) { continue 'l }
                    v.push(f);
                }
                let v1: Vec<_> = v.iter().map(|y| *y >> 1).collect();

                let mut vs = vec![0; 30];
                for i in 0..5 {
                    let b = 3 * (v1[w[i][0]] - 1) as usize;
                    for j in 0..3 {
                        vs[b + j] = v1[w[i][j]];
                    }
                }
                if !rs.contains(&vs) { rs.push(vs); }
            }

            for r in rs {
                let mut s = "".to_string();
                for v in r {
                    if v == 0 { continue }
                    s.push_str(v.to_string().as_str())
                }
                if s.len() == 16 { res.push(s); }
            }
        }
    }
    res.sort_unstable();
    // println!("{res:?}");
    res[res.len() - 1].clone()
}