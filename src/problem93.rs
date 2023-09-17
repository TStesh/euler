use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum TokenType {
    LeftBracket, RightBracket, OpPlus, OpMinus, OpMul, OpDiv, NUMBER, EOF
}

#[derive(Debug)]
struct Token {
    lex_type: TokenType,
    lex_num_value: f64,
    lex_str_value: String
}

impl Token {
    fn null() -> Self { Self { lex_type: TokenType::EOF,
        lex_num_value: 0., lex_str_value: "".to_string() } }
    fn oper(t: TokenType) -> Self { Self { lex_type: t,
        lex_num_value: 0., lex_str_value: "".to_string() } }
    fn num(n: f64) -> Self { Self { lex_type: TokenType::NUMBER,
        lex_num_value: n, lex_str_value: "".to_string() } }
}

#[derive(Debug)]
struct Formula {
    items: Vec<Token>
}

impl Formula {
    // Разбиваем выражение на лексемы
    fn new(s: &str) -> Self {
        let mut items = vec![];
        let mut i = 0;
        while i < s.len() {
            let c = FromStr::from_str(&s[i..i + 1]).unwrap();
            let mut t = Token::null();
            match c {
                '+' => t = Token::oper(TokenType::OpPlus),
                '-' => t = Token::oper(TokenType::OpMinus),
                '*' => t = Token::oper(TokenType::OpMul),
                '/' => t = Token::oper(TokenType::OpDiv),
                '(' => t = Token::oper(TokenType::LeftBracket),
                ')' => t = Token::oper(TokenType::RightBracket),
                '1'..='9' => t = Token::num(c.to_string().parse::<f64>().unwrap()),
                ' ' => { i += 1; continue; },
                _ => ()
            }
            items.push(t);
            i += 1;
        }
        items.push(Token::null());
        Self { items }
    }
}

struct Calc {
    tokens: Formula,
    pos: usize
}

impl Calc {
    fn new(s: &str) -> Self {
        let tokens = Formula::new(s);
        Self { tokens, pos: 0 }
    }

    fn calc_expr(&mut self) -> f64 {
        if self.tokens.items[self.pos].lex_type != TokenType::EOF {
            self.calc_plusminus()
        } else { 0. }
    }

    fn calc_plusminus(&mut self) -> f64 {
        let mut x = self.calc_muldiv();
        loop {
            match self.tokens.items[self.pos].lex_type {
                TokenType::OpPlus => {
                    self.pos += 1;
                    x += self.calc_muldiv();
                },
                TokenType::OpMinus => {
                    self.pos += 1;
                    x -= self.calc_muldiv();
                },
                _ => return x
            }
        }
    }

    fn calc_muldiv(&mut self) -> f64 {
        let mut x = self.calc_factor();
        loop {
            match self.tokens.items[self.pos].lex_type {
                TokenType::OpMul => {
                    self.pos += 1;
                    x *= self.calc_factor();
                },
                TokenType::OpDiv => {
                    self.pos += 1;
                    x /= self.calc_factor();
                },
                _ => return x
            }
        }
    }

    fn calc_factor(&mut self) -> f64 {
        match self.tokens.items[self.pos].lex_type {
            TokenType::NUMBER => {
                let x = self.tokens.items[self.pos].lex_num_value;
                self.pos += 1;
                return x
            },
            TokenType::LeftBracket => {
                self.pos += 1;
                let x = self.calc_expr();
                if self.tokens.items[self.pos].lex_type == TokenType::RightBracket {
                    self.pos += 1;
                    return x
                } else { 0. }
            },
            _ => 0.
        }
    }
}

fn gen_expr(arr: &[u8]) -> u32 {
    let patterns = vec![
        "1?2?3?4",
        "1?2?(3?4)",
        "1?(2?3)?4",
        "1?(2?3?4)",
        "1?(2?(3?4))",
        "1?((2?3)?4)",
        "(1?2)?3?4",
        "(1?2)?(3?4)",
        "(1?2?3)?4",
        "(1?(2?3))?4",
        "((1?2)?3)?4"
    ];
    let mut mm = HashMap::new();
    for a in euler::permutations(arr) {
        for c1 in ['+', '-', '*', '/'] {
            let x1 = c1.to_string();
            for c2 in ['+', '-', '*', '/'] {
                let x2 = c2.to_string();
                for c3 in ['+', '-', '*', '/'] {
                    let x3 = c3.to_string();
                    let oper = [x1.clone(), x2.clone(), x3.clone()];
                    for p in &patterns {
                        let mut e = "".to_string();
                        let mut op_num = 0;
                        for c in p.chars() {
                            match c {
                                '1' => e.push_str(a[0].to_string().as_str()),
                                '2' => e.push_str(a[1].to_string().as_str()),
                                '3' => e.push_str(a[2].to_string().as_str()),
                                '4' => e.push_str(a[3].to_string().as_str()),
                                '?' => {
                                    e.push_str(oper[op_num].as_str());
                                    op_num += 1;
                                },
                                _ => e.push_str(c.to_string().as_str())
                            }
                        }
                        let mut xs = Calc::new(&e);
                        let ys = xs.calc_expr();
                        if ys.fract() == 0. && ys > 0. && !mm.contains_key(&(ys as u32)) {
                            mm.insert(ys as u32, e);
                        }
                    }
                }
            }
        }
    }
    let mut v: Vec<_> = mm.into_keys().collect();
    v.sort_unstable();
    let mut res = 0;
    for (i, k) in v.iter().enumerate() {
        if *k != i as u32 + 1 && i > 0 {
            res = v[i - 1];
            break;
        }
    }
    res
}

pub fn problem93() -> (String, u32) {
    let mut v = vec![];
    let mut max_r = 0;
    let mut idx = 0;
    let mut k = 0;
    for a in 1..=6 {
        for b in 2..=7 {
            if b <= a { continue };
            for c in 3..=8 {
                if c <= b || c <= a { continue };
                for d in 4..=9 {
                    if d <= c || d <= b || d <= a { continue };
                    let r = gen_expr(&[a, b, c, d]);
                    if r > max_r {
                        max_r = r;
                        idx = k;
                    };
                    k += 1;
                    v.push((format!("({a}, {b}, {c}, {d})"), r));
                }
            }
        }
    }
    (v[idx].0.clone(), v[idx].1)
}

/*
mod problems93;
use crate::problems93::problem93;

fn main() {
    let start = std::time::Instant::now();
    let (x, y) = problem93();
    println!("{x}: {y}");
    println!("Duration: {:?}", start.elapsed());
}
*/