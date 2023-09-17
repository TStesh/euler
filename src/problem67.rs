struct Triangle { t: Vec<Vec<i32>> }

impl Triangle {
    fn new() -> Self {
        let mut t = vec![];
        let xs = include_str!("c:\\users\\alexa\\downloads\\p067_triangle.txt");
        for line in xs.lines() {
            let mut r = vec![];
            for x in line.split(' ') {
                if x.len() > 0 && x != " " {
                    r.push(x.parse::<i32>().unwrap());
                }
            }
            t.push(r);
        }
        Self { t }
    }

    fn size(&self) -> usize { self.t.len() }

    fn path(&self) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        // массив длин путей от вершины до каждого элемента
        let mut l = vec![vec![self.t[0][0]]];
        // массив направлений: 1 = up, -1 = diagonal
        let mut d = vec![vec![0]];
        for r in 1..self.size() {
            let size= self.t[r].len();
            let mut xs = Vec::with_capacity(size);
            unsafe { xs.set_len(size) }
            let mut ys = Vec::with_capacity(size);
            unsafe { ys.set_len(size) }
            ys.fill(1);
            // обрабатываем первый узел
            xs[0] = l[r - 1][0] + self.t[r][0];
            // обрабатываем последний узел
            let idx = size - 1;
            xs[idx] = l[r - 1][idx - 1] + self.t[r][idx];
            ys[idx] = -1;
            // обрабатываем промежуточные узлы
            for c in 1..idx {
                if l[r - 1][c - 1] > l[r - 1][c] {
                    xs[c] = l[r - 1][c - 1] + self.t[r][c];
                    ys[c] = -1;
                } else {
                    xs[c] = l[r - 1][c] + self.t[r][c];
                    ys[c] = 1;
                }
            }
            l.push(xs);
            d.push(ys);
        }
        (l, d)
    }
}

/*
fn main() {

    let tr = Triangle::new();

    let (xs, ys) = tr.path();
    let path_max_len = *xs[tr.size() - 1].iter().max().unwrap();
    println!("path max len = {path_max_len}");
}
*/
