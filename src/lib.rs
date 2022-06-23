#![allow(unused)]

use std::cmp::min;
use std::collections::HashMap;

pub fn wheel_sieve(n: u64, include_2: bool) -> Vec<u64> {
    let limit = n as usize;
    let idx = [1usize, 7, 11, 13, 17, 19, 23, 29];
    let mut primes: Vec<u64> = Vec::with_capacity(n as usize);
    if include_2 {
        primes.push(2u64)
    }
    primes.push(3u64);
    primes.push(5u64);
    // инициализация массива смещений в разрезе байтов
    let mut offsets_per_byte = [[0usize; 8]; 256];
    for b in 0..256 {
        for i in 0..8 {
            if b & (1 << i) > 0 {
                offsets_per_byte[b][i] = idx[i]
            }
        }
    }
    // решето
    let mut set = vec![255u8; 1 + (limit / 30)];
    let mut bit = [8u8; 30];
    for i in 0..8 {
        bit[idx[i]] = i as u8
    }
    let mut q = 7;
    let mut sqr = 49;
    let mut x;
    let mut y;
    // попробовать реализовать быстрое деление на 30!!!
    while sqr <= limit {
        x = bit[q % 30];
        if x < 8 && (set[q / 30] & (1 << x)) > 0 {
            for i in (sqr..limit).step_by(q << 1) {
                y = bit[i % 30];
                if y < 8 {
                    set[i / 30] &= !(1 << y)
                }
            }
        }
        sqr += (q + 1) << 2;
        q += 2;
    }
    // простые
    for i in (7..limit).step_by(2) {
        x = bit[i % 30];
        if x < 8 && (set[i / 30] & (1 << x)) > 0 {
            primes.push(i as u64)
        }
    }
    primes
}

// Cегментированный Эратосфен
pub fn segment_sieve(n: u64, include_2: bool) -> Vec<u64> {
    let limit = n as usize;
    const BUF_SIZE: usize = 49_152;
    let mut fin_primes: Vec<u64> = Vec::with_capacity(limit);
    if include_2 { fin_primes.push(2) }
    for p in [3u64, 5, 7, 11, 13, 17, 19, 23, 29] {
        fin_primes.push(p)
    }
    let length = 1 + ((limit as f64).powf(0.5) as usize);
    // инициализация массива смещений в разрезе байтов
    let idx = [1usize, 7, 11, 13, 17, 19, 23, 29];
    let mut offsets_per_byte = [[0usize; 8]; 256];
    for b in 0..256 {
        for i in 0..8 {
            if b & (1 << i) > 0 {
                offsets_per_byte[b][i] = idx[i]
            }
        }
    }
    // Get initial primes
    let mut set = vec![255u8; 1 + (length / 30)];
    let mut bit = [8u8; 30];
    for i in 0..8 {
        bit[idx[i]] = i as u8
    }
    let (mut q, mut sqr, mut x, mut y) = (7usize, 49usize, 0u8, 0u8);
    while sqr <= length {
        x = bit[q % 30];
        if x < 8 && (set[q / 30] & (1 << x)) > 0 {
            for i in (sqr..length).step_by(q << 1) {
                y = bit[i % 30];
                if y < 8 {
                    set[i / 30] &= !(1 << y)
                }
            }
        }
        sqr += (q + 1) << 2;
        q += 2;
    }
    // работаем с полученными простыми
    let mut primes = Vec::with_capacity(length);
    let mut prime_multiples: Vec<Vec<usize>> = vec![Vec::with_capacity(length); 8];
    let (mut prime_count, mut val, mut rem, mut offset, mut byte, mut p);
    prime_count = 0usize;
    sqr = 49;
    for i in (7..length).step_by(2) {
        x = bit[i % 30];
        if x < 8 && (set[i / 30] & (1 << x)) > 0 {
            // i - простое число, sqr - его квадрат
            primes.push(i);
            prime_count += 1;
            // для простого числа нужно посчитать начальные смещения
            offset = i << 1;
            val = sqr;
            byte = 0u8;
            while byte != 255 {
                rem = val % 30;
                match rem {
                    1 | 7 | 11 | 13 | 17 | 19 | 23 | 29 => {
                        p = bit[rem] as usize;
                        prime_multiples[p].push((val - rem) / 30);
                        byte |= 1 << p;
                    }
                    _ => (),
                }
                val += offset;
            }
        }
        sqr += (i + 1) << 2;
    }
    // подготовка сегментов
    let mut segment_data;
    let max = (limit + 29) / 30;
    // границы сегмента
    let mut seg_start = 1usize;
    let mut seg_end = min(seg_start + BUF_SIZE, max);
    let (mut prime, mut seg_len, mut x, mut y, mut mask, mut data);
    // цикл по сегментам
    while seg_start < max {
        // инициализация
        segment_data = [255u8; BUF_SIZE];
        seg_len = seg_end - seg_start;
        // было бы классно сделать это в 8 потоков
        for i in 0..8 {
            mask = !(1 << i) as u8;
            // бежим по всем нужным нам простым числам
            for j in 0..prime_count {
                prime = primes[j];
                // вычеркиваем кратные
                x = prime_multiples[i][j] - seg_start;
                if x >= seg_len {
                    continue;
                }
                while x < seg_len {
                    segment_data[x] &= mask;
                    x += prime;
                }
                prime_multiples[i][j] = x + seg_start;
            }
        }
        // выбираем простые числа из сегмента
        'l: for i in 0..BUF_SIZE {
            offset = (seg_start + i) * 30;
            data = segment_data[i] as usize;
            for j in 0..8 {
                y = offsets_per_byte[data][j];
                if y == 0 {
                    continue;
                }
                x = offset + y;
                if x >= limit {
                    break 'l;
                }
                fin_primes.push(x as u64);
            }
        }
        seg_start = seg_end;
        seg_end = min(seg_start + BUF_SIZE, max);
    }
    fin_primes
}

// вычисление тотиентов для чисел на интервале [1..n)
// Для миллиарда считает порядка 31 сек
pub fn totient(n: u64, primes: &[u64]) -> Vec<u64> {
    let mut tot: Vec<u64> = Vec::with_capacity(n as usize);
    // Заполняем вектор тотиентов единицами
    unsafe { tot.set_len(n as usize) }
    tot.fill(1);
    //for _ in 0..n { tot.push(1) }
    // заполняем тотиенты степеней простых
    let mut prime: u64;
    let mut u: u64;
    let mut v: u64;
    let mut z: u64;
    let mut k: u64;
    // цикл для 2
    u = 1;
    v = 2;
    while v < n {
        for x in (v..n).step_by((v << 1) as usize) { tot[x as usize] = u; }
        u = v;
        v <<= 1;
    }
    // цикл для остальных простых
    for p in primes.iter() {
        prime = *p;
        u = 1;
        v = prime;
        while v < n {
            z = v - u;
            // считаем тотиент для степени простого
            tot[v as usize] = z;
            // считаем тотиенты для кратных степени простого числа
            // пропускаем только кратные более высокой степени простого числа
            k = 2;
            for x in ((v << 1)..n).step_by(v as usize) {
                if k == prime {
                    k = 1;
                    continue;
                }
                tot[x as usize] *= z;
                k += 1;
            }
            u = v;
            v *= prime;
        }
    }
    tot
}

// вычисление сумм делителей для чисел на интервале [1..n)
pub fn vec_sigma(n: u64, primes: &[u64]) -> Vec<u64> {
    let mut sig: Vec<u64> = Vec::with_capacity(n as usize);
    // Заполняем вектор сигм единицами
    unsafe { sig.set_len(n as usize) }
    sig.fill(1);
    //for _ in 0..n { sig.push(1) }
    // Считаем сигмы степеней простых
    let mut prime: u64;
    let mut u: u64;
    let mut v: u64;
    let mut k: u64;
    let mut counter = 0u64;
    // цикл для 2 - приходится обрабатывать отдельно
    u = 1;
    v = 2;
    while v < n {
        u += v;
        for x in (v..n).step_by((v << 1) as usize) {
            sig[x as usize] = u;
        }
        v <<= 1;
    }
    // цикл для остальных простых
    for p in primes.iter() {
        prime = *p;
        u = 1;
        v = prime;
        while v < n {
            u += v;
            // считаем сигму для степени простого
            sig[v as usize] = u;
            // считаем сигмы для кратных степени простого числа
            // пропускаем только кратные более высокой степени простого числа
            k = 2;
            for x in ((v << 1)..n).step_by(v as usize) {
                if k == prime {
                    k = 1;
                    continue;
                }
                sig[x as usize] *= u;
                k += 1;
            }
            v *= prime;
        }
    }
    sig
}

// вычисление сумм делителей для чисел на интервале [1..n)
pub fn mod_sigma(n: u64, primes: &[u64], hp: &HashMap<u64, u64>) -> Vec<u64> {
    let mut sig: Vec<u64> = Vec::with_capacity(n as usize);
    // Заполняем вектор сигм единицами
    for _ in 0..n { sig.push(1) }
    // Считаем сигмы степеней простых
    let (mut prime, mut u, mut u_prev, mut v, mut k, mut z, mut factor);
    // цикл для 2 - приходится обрабатывать отдельно
    u = 1;
    v = 2;
    while v < n {
        u_prev = u;
        u += v;
        z = u + (u_prev << 1);
        for x in (v..n).step_by((v << 1) as usize) {
            sig[x as usize] = z;
        }
        v <<= 1;
    }
    // цикл для остальных простых
    for p in primes.iter() {
        prime = *p;
        u = 1;
        v = prime;
        factor = 0;
        if hp.contains_key(&p) { factor = *hp.get(&p).unwrap() }
        while v < n {
            u_prev = u;
            u += v;
            // считаем сигму для степени простого
            z = u + factor * u_prev;
            sig[v as usize] = z;
            // считаем сигмы для кратных степени простого числа
            // пропускаем только кратные более высокой степени простого числа
            k = 2;
            for x in ((v << 1)..n).step_by(v as usize) {
                if k == prime {
                    k = 1;
                    continue;
                }
                sig[x as usize] *= z;
                k += 1;
            }
            v *= prime;
        }
    }
    sig
}

// вычисление радикалов на интервале [1..n)
pub fn vec_rad(n: u64, primes: &[u64]) -> Vec<u64> {
    let mut rad: Vec<u64> = Vec::with_capacity(n as usize);
    for _ in 0..n { rad.push(1) }
    let mut prime: u64;
    let mut v: u64;
    for p in primes.iter() {
        prime = *p;
        v = prime;
        rad[v as usize] = prime;
        for x in ((v << 1)..n).step_by(v as usize) {
            rad[x as usize] *= prime
        }
    }
    rad
}

// Находим пары (a, b): a²+b² = простое число
// В хэш записываем выражение 2 * (a + b)
pub fn gen_pairs(n: u64, primes: &[u64]) -> HashMap<u64, u64> {
    let mut hh = HashMap::new();
    let mut p: u64;
    // работаем только с простыми вида 4m + 1
    for prime in primes.iter() {
        p = *prime;
        if (p - 1) & 3 == 0 { hh.insert(p, 0u64); }
    }
    let (mut u, mut u2, mut v, mut v2) = (1u64, 1u64, 2u64, 4u64);
    let mut z: u64;
    let mut k = 1u64;
    while u2 < n {
        (v, v2, k) = (u + 1, u2 + (u << 1) + 1, 1);
        z = u2 + v2;
        while z < n {
            z = u2 + v2;
            if let Some(x) = hh.get_mut(&z) { *x = (u + v) << 1; }
            v2 += (v << 1) + 1;
            v += 1;
            // u и v должны быть взаимно просты
            // для этого вводим k, который пробегает значения от 1 до u
            // если k = u, пропускаем шаг и возвращаем k в 1.
            k += 1;
            if k == u {
                v2 += (v << 1) + 1;
                v += 1;
                k = 1;
            }
        }
        u2 += (u << 1) + 1;
        u += 1;
    }
    hh
}

// Простейшая проверка на простоту
pub fn is_prime(n: u64) -> bool {
    if n & 1 == 0 { return n == 2 }
    if n % 3 == 0 { return n == 3 }
    let mut d = 5;
    while d * d < n {
        if n % d == 0 || n % (d + 2) == 0 { return false }
        d += 6;
    }
    true
}

// Наибольший общий делитель (классический метод)
pub fn old_gcd(n: u64, m: u64) -> u64 {
    if n == m { return n }
    let (mut a, mut b) = if n < m { (m, n) } else { (n, m) };
    if a % b == 0 { return b }
    while b > 0 { (a, b) = (b, a % b) }
    a
}

// Наибольший общий делитель (бинарный метод)
// Скорость плюс-минус равна классическому, но нет операций деления!
pub fn gcd(n: u64, m: u64) -> u64 {
    if n == m { return n }
    let (mut u, mut v) = (n, m);
    let (mut a, mut b) = (0u64, 0u64);
    if u & 1 == 0 { a = u.trailing_zeros() as u64; u >>= a; }
    if v & 1 == 0 { b = v.trailing_zeros() as u64; v >>= b; }
    if (u == 1 && v > 1) || (v == 1 && u > 1) { return 1 << min(a, b) }
    while u != v {
        (u, v) = (u.abs_diff(v), u);
        if u & 1 == 0 { u >>= u.trailing_zeros(); }
        if u == 1 && v > 1 { return 1 << if a > b { b } else { a } }
    }
    u << if a > b { b } else { a }
}

// Взаимно просты?
// n > m
pub fn coprime(n: u64, m: u64) -> bool {
    if n == m || n % m == 0 { return false }
    let (mut a, mut b) = (n, m);
    while b > 0 { (a, b) = (b, a % b) }
    a == 1
}

// Радикал
// маcсив primes должен начинаться с 2
pub fn rad(num: u64, primes: &[u64]) -> u64 {
    if num == 1 { return 1 }
    let mut rad = 1;
    let mut n = num;
    let mut prime: u64;
    for p in primes.iter() {
        prime = *p;
        if n >= prime && n % prime == 0 {
            rad *= prime;
            n /= prime;
            while n >= prime && n % prime == 0 {
                n /= prime
            }
        }
        if n == 1 { break }
    }
    rad
}

// Стпень δ(p) простого числа в разложении n!
pub fn exp(n: u64, p: u64) -> u64 {
    let mut exp = 0u64;
    let mut exp_p = p;
    while exp_p < n {
        exp += n / exp_p;
        exp_p *= p;
    }
    exp
}

// Считаем p^m % b
// Степень считаем рекурсивно -> странно, но факт - рекурсивно считает быстрее чем итеративно
pub fn powmod(p: u64, m: u64, b: u64) -> u64 {
    let x = p % b;
    if m == 1 { return x }
    let y = powmod(x * x,m >> 1, b);
    if m & 1 == 0 { y % b } else { (x * y) % b }
}

// Вычисление порядка числа 10 в группе Z(p)
pub fn ord(num: u64, primes: &[u64]) -> u64 {
    let n = num - 1;
    let x = n >> 1;
    let mut p: u64;
    let mut z: u64;
    for prime in primes.iter() {
        p = *prime;
        if p > x { break }
        if n % p == 0 {
            z = powmod(10, p , num);
            //println!("p={p} z={z}");
            if z == 1 { return p }
            if z == n { return p << 1 }
        }
    }
    n
}

// Вычисление степени матрицы 2х2 в кольце вычетов Z/mZ
// Начальную матрицу задаем массивом из 4 чисел
pub fn exp_mod_matrix(u: [u64; 4], n: u64, m: u64) -> [u64; 4] {
    // лямбда для вычисления произведения двух 2х2 матриц
    let f = |a: [u64; 4], b: [u64; 4]| [
        (a[0] * b[0] + a[1] * b[2]) % m, (a[0] * b[1] + a[1] * b[3]) % m,
        (a[2] * b[0] + a[3] * b[2]) % m, (a[2] * b[1] + a[3] * b[3]) % m
    ];
    if n == 0 { return [1, 0, 0, 1] }
    if n == 1 { return u }
    let x = exp_mod_matrix(f(u, u), n >> 1, m);
    return if n & 1 == 0 { x } else { f(x, u) }
}


// Генератор перестановок n числовых объектов
// Итерационный алгоритм:
pub fn permutations<T: Copy + PartialEq>(arr: &[T]) -> Vec<Vec<T>> {
    let ls = arr.len();
    let mut ps: Vec<Vec<T>> = arr.iter().map(|x| vec![*x]).collect();
    if ls < 2 { return ps }
    for _ in 1..ls {
        let mut ps_new: Vec<Vec<T>> = Vec::new();
        for p in ps {
            for a in arr {
                let mut p_new = p.clone();
                if !p.contains(a) {
                    p_new.push(*a);
                    ps_new.push(p_new);
                }
            }
        }
        ps = ps_new
    }
    ps
}

// Цифры числа
pub fn num_digits(n: u64) -> Vec<u8> {
    (1..).into_iter().scan(n, |s, _| {
        let y = (*s % 10) as u8;
        *s /= 10;
        if *s > 0 || y > 0 { Some(y) } else { None }
    }).fuse().collect()
}

// Число из цифр
pub fn digits_num(d: &[u8]) -> u64 {
    d.iter().scan(1, |s, &x| {
        let y = *s * x as u64; *s *= 10; Some(y)
    }).sum()
}

// Максимальный элемент и индекс максимального элемента
pub fn max<T: Copy + PartialOrd>(v: &[T]) -> (T, usize) {
    let mut m = v[0];
    let mut index = 0usize;
    for vs in v.iter().enumerate() {
        if *vs.1 > m {
            m = *vs.1;
            index = vs.0;
        }
    }
    (m, index)
}