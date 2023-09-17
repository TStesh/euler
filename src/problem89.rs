pub fn problem89() -> usize {
    let xs = include_str!("c:\\users\\alexa\\downloads\\p089_roman.txt");
    let mut ys = 0;
    for line in xs.lines() {
        let y = euler::decimal_to_roman(euler::roman_to_decimal(line));
        if line.len() < y.len() {
            println!("oops: {line} < {y}");
        } else {
            ys += line.len() - y.len();
        }
    }
    ys
}