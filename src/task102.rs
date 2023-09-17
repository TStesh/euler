pub fn task102() -> usize {
    let xs = include_str!("c:\\users\\alexa\\downloads\\p102_triangles.txt");
    let mut count = 0usize;
    for (i, line) in xs.lines().enumerate() {
        let mut r = vec![];
        for x in line.split(',') {
            if x.len() > 0 && x != " " {
                r.push(x.parse::<i32>().unwrap());
            }
        }
        // r0=Xa, r1=Ya, r2=Xb, r3=Yb, r4=Xc, r5=Yc
        // p1=XaYb - XbYa
        let p1 = r[0] * r[3] - r[2] * r[1];
        // p2=XbYc - XcYb
        let p2 = r[2] * r[5] - r[4] * r[3];
        // p3=XcYa - XaYc
        let p3 = r[4] * r[1] - r[0] * r[5];
        if (p1 >= 0 && p2 >= 0 && p3 >= 0) ||
            (p1 <= 0 && p2 <= 0 && p3 <= 0) {
            println!("#{i}: {line} contains the origin");
            count += 1;
        } else {
            println!("#{i}: {line} not contains the origin");
        }
    }
    count
}
