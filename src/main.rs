fn task(num: u64, primes: &Vec<u64>) -> u64 {
    let e = euler::exp(num, 2) - euler::exp(num, 5);
    let mut r = euler::powb_mod(2, e, 100_000);
    for prime in primes {
        let p = *prime;
        if p == 5 { continue }
        if p > num { break }
        r *= euler::powb_mod(p, euler::exp(num, p), 100_000);
        r %= 100_000;
    }
    r
}

fn main() {
    let start = std::time::Instant::now();
    let num = 10_000_000u64;
    let e = euler::exp(num, 2) - euler::exp(num, 5);
    let mut r = euler::powb_mod(2, e, 100_000);
    for num in (3..num).step_by(2) {
        if num % 5 == 0 { continue }
        r *= num % 100_000
    }
    println!("res: {tot}");
    println!("Duration: {:?}", start.elapsed());
}
