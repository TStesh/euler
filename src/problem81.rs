struct Matrix { m: Vec<Vec<i32>> }

impl Matrix {
    fn new() -> Self {
        let mut m = vec![];
        let xs = include_str!("c:\\users\\alexa\\downloads\\p081_matrix.txt");
        for line in xs.lines() {
            let mut r = vec![];
            for x in line.split(',') {
                if x.len() > 0 && x != " " {
                    r.push(x.parse::<i32>().unwrap());
                }
            }
            if r.len() > 0 { m.push(r); }
        }
        // println!("{m:?}");
        Self { m }
    }

    fn size(&self) -> usize { self.m.len() }

    fn path(&self) -> Vec<i32> {
        // массив длин путей от вершины до каждого элемента
        let mut l = vec![vec![self.m[0][0]]];
        // Обрабатываем первую строку:
        let size = self.m[0].len();
        for c in 1..size {
            let ys = l[0][c - 1] + self.m[0][c];
            l[0].push(ys);
        }
        // Обрабатываем следующи строки:
        let rows = self.m.len();
        for r in 1..rows {
            let mut xs = Vec::with_capacity(size);
            unsafe { xs.set_len(size) }
            // обрабатываем первый узел
            xs[0] = l[r - 1][0] + self.m[r][0];
            // обрабатываем промежуточные узлы
            for c in 1..size {
                let a = xs[c - 1];
                let b = l[r - 1][c];
                xs[c] = self.m[r][c] + if a < b { a } else { b };
            }
            l.push(xs);
        }
        l[rows - 1].clone()
    }
}

pub fn problem81() -> u32 {
    let mx = Matrix::new();
    let path = mx.path();
    path[path.len() - 1]
}
