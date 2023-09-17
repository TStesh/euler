struct Matrix {
    m: Vec<Vec<i32>>,
    l: Vec<Vec<(f64, bool)>>
}

impl Matrix {
    fn new() -> Self {
        let mut m = vec![];
        let xs = include_str!("c:\\users\\alexa\\downloads\\p082_matrix.txt");
        for line in xs.lines() {
            let mut r = vec![];
            for x in line.split(',') {
                if x.len() > 0 && x != " " {
                    r.push(x.parse::<i32>().unwrap());
                }
            }
            if r.len() > 0 { m.push(r); }
        }
        let l = vec![vec![(f64::INFINITY, false); m[0].len()]; m.len()];
        // println!("rows = {}, columns = {}", m.len(), m[0].len());
        Self { m, l }
    }

    // rs - номер строки стартового элемента
    // примитивная реализация алгоритма Дейкстры
    fn path(&self, rs: usize) -> u32 {
        let row_num = self.m.len();
        let col_num = self.m[0].len();
        let mut ll = self.l.clone();
        ll[rs][0].0 = self.m[rs][0] as f64;

        loop {
            // ищем подходящий узел (минимальная метка, флаг прохождения = false):
            // если не нашли ни одного, выходим из цикла
            let mut min_value = f64::INFINITY;
            let mut min_row_num = 0;
            let mut min_col_num = 0;
            let mut found = false;
            // этот цикл занимает почти все время
            // чтобы заменить его на что-то более эффективное, придуманы кучи
            // на вершине кучи всегда будет минимальный элемент, а вставка
            // элемента в кучу занимает время О(logN)
            for i in 0..row_num {
                for j in 0..col_num {
                    if !ll[i][j].1 && ll[i][j].0 < min_value {
                        found = true;
                        min_value = ll[i][j].0;
                        min_row_num = i;
                        min_col_num = j;
                    }
                }
            }
            if !found { break }
            // метка текущего узла:
            let curr_lbl = ll[min_row_num][min_col_num].0;
            // промежуточные массивы:
            let a = [min_col_num, 0, min_row_num];
            let b = [col_num - 1, min_row_num, row_num - 1];
            let c = [min_row_num + 1, min_row_num, min_row_num + 2];
            let d = [min_col_num + 1, min_col_num, min_col_num];
            // анализ соседних вершин:
            for k in 0..3 {
                if a[k] < b[k] && !ll[c[k] - 1][d[k]].1 {
                    let i = c[k] - 1;
                    let j = d[k];
                    // есть непросмотренный сосед справа!
                    let nbh_lbl = ll[i][j].0;
                    let new_lbl = curr_lbl + self.m[i][j] as f64;
                    // проверяем метку соседа на минимальность:
                    ll[i][j].0 = if nbh_lbl < f64::INFINITY {
                        // если метка соседа < ∞, то сравнить
                        // текущее значение метки соседа с новым = curr_lbl + nom:
                        if new_lbl < nbh_lbl { new_lbl } else { nbh_lbl }
                    } else { new_lbl };
                }
            }
            // меняем флаг прохождения текущего узла:
            ll[min_row_num][min_col_num].1 = true;
        }
        // анализируем результат
        (0..row_num).map(|i| ll[i][col_num - 1].0 as u32).min().unwrap()
    }
}

pub fn problem82() -> u32 {
    let xs = Matrix::new();
    (0..xs.m.len()).map(|r| xs.path(r)).min().unwrap()
}
