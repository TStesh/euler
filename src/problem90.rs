// нормализация
fn normalize(s: &Vec<u8>) -> Vec<u8> {
    let mut v = [0; 10];
    for i in 0..6 { v[s[i] as usize] = 1; }
    if v[9] == 1 {
        v[9] = 0;
        if v[6] == 0 { v[6] = 1; }
    }
    if v[7] == 1 { v[7] = 0; }
    let mut r = vec![];
    for i in 0..10 {
        if v[i] == 1 { r.push(i as u8); }
    }
    r
}

// проверка, что покрываются квадраты
fn check(s1: &Vec<u8>, s2: &Vec<u8>) -> bool {
    let mut v = vec![];
    let sqr = [(0, 1), (0, 4), (0, 6), (1, 6), (1, 8), (2, 5), (3, 6), (4, 6)];
    let ns1 = normalize(s1);
    let ns2 = normalize(s2);
    // формируем пары
    for i in 0..ns1.len() {
        for j in 0..ns2.len() {
            let (x, y) = (ns1[i], ns2[j]);
            let mut pair = if x < y { (x, y) } else { (y, x) };
            if !v.contains(&pair) { v.push(pair); }
        }
    }
    // проверяем на полноту
    let mut full = true;
    for p in sqr {
        if !v.contains(&p) {
            full = false;
            break;
        }
    }
    full
}

// тупой генератор шестерок (но умнее и не нужен, их всего 210 штук)
fn gen_six() -> Vec<Vec<u8>> {
    let mut v = vec![];
    for i1 in 0..10 {
        for i2 in i1 + 1..10 {
            for i3 in i2 + 1..10 {
                for i4 in i3 + 1..10 {
                    for i5 in i4 + 1..10 {
                        for i6 in i5 + 1..10 {
                            v.push(vec![i1, i2, i3, i4, i5, i6]);
                        }
                    }
                }
            }
        }
    }
    v
}

pub fn problem90() -> i32 {
    let v = gen_six();
    let mut count = 0;
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            if check(&v[i], &v[j]) {
                count += 1;
                // println!("#{count}: {:?}, {:?}", v[i], v[j]);
            }
        }
    }
    count
}
