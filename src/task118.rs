/// Используя все цифры от 1 до 9 и свободно объединяя их для образования десятичных целых чисел,
/// можно формировать различные наборы. Интересно, что в наборе {2,5,47,89,631} все принадлежащие
/// ему элементы являются простыми. Сколько различных множеств, содержащих каждую из цифр от 1 до 9
/// ровно один раз, содержит только простые элементы?
use euler::{is_prime, permutations};

// Представление 9 в виде сумм не уменьшающихся слагаемых
fn part() -> Vec<Vec<u8>> {
    let mut r = vec![vec![1], vec![2], vec![3], vec![4]];
    let mut run = true;
    while run {
        let mut r_new = vec![];
        run = false;
        for v in r {
            let last = v[v.len() - 1];
            let tot: u8 = 9 - v.iter().sum::<u8>();
            if tot == 0 {
                r_new.push(v.clone());
                continue
            }
            if tot >= last {
                run = true;
                for k in last..=tot {
                    let mut vc = v.clone();
                    vc.push(k);
                    r_new.push(vc);
                }
            }
        }
        r = r_new;
    }
    r
}

fn gen_sets(pat: &Vec<u8>) -> Vec<Vec<u64>> {
    let mut r = vec![];
    'l: for xs in permutations(&[1u8, 2, 3, 4, 5, 6 ,7, 8, 9]) {
        let mut v = vec![];
        let mut k = 0usize;
        for item in pat {
            let dk = *item as usize;
            let y = &xs[k..k + dk];
            if (*item == 1 && y[0] == 1) || (*item > 1 && y[0] & 1 == 0) { continue 'l }
            let z = euler::digits_num(&y);
            if is_prime(z) { v.push(z); } else { continue 'l }
            k += dk;
        }
        v.sort_unstable();
        if !r.contains(&v) {
            r.push(v);
        }
    }
    r
}

pub fn task118() -> usize {
    let mut total = 0;
    for xs in part() {
        if xs.iter().filter(|x| **x == 1).count() > 4 { continue }
        total += gen_sets(&xs).len();
    }
    total
}