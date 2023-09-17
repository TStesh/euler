fn check(num: u64) -> (u64, u64) {
    if num == 5 { return (1, 2) }
    if num < 55 || !euler::palindromyc(num) { return (0, 0) }
    let mut n = 2;
    while n * (n + 1) * ((n << 1) + 1) <= 6 * num {
        let x = n * (n - 1);
        let d = n * ((num << 2) - x * (n + 1) / 3);
        let sqr_d = (d as f64).powf(0.5) as u64;
        if sqr_d * sqr_d == d {
            let a = sqr_d - x;
            if a % (n << 1) == 0 { return (a / (n << 1), n) }
        }
        n += 1;
    }
    (0, 0)
}

pub fn task125() {
    let start = std::time::Instant::now();
    let mut count = 0;
    for num in 1..100_000_000 {
        let (a, n) = check(num);
        if a > 0 { count += num; }
    }
    println!("Duration: {:?}", start.elapsed());
    println!("{count}");
}
