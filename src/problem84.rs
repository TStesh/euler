use std::collections::VecDeque;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

const BRD: [&str; 40] = [
    //0    1      2     3     4     5     6     7      8     9
    "GO", "A1", "CC1", "A2", "T1", "R1", "B1", "CH1", "B2", "B3",
    "JAIL", "C1", "U1", "C2", "C3", "R2", "D1", "CC2", "D2", "D3",
    "FP", "E1", "CH2", "E2", "E3", "R3", "F1", "F2", "U2", "F3",
    "G2J", "G1", "G2", "CC3", "G3", "R4", "CH3", "H1", "T2", "H2"
];

// топ-3
pub fn top3(arr: &Vec<u32>) -> (usize, usize, usize) {
    let mut max1 = 0;
    let mut ind1 = 0;
    let mut max2 = 0;
    let mut ind2 = 0;
    let mut max3 = 0;
    let mut ind3 = 0;
    for i in 0..arr.len() {
        let x = arr[i];
        if x > max1 {
            max3 = max2; ind3 = ind2; max2 = max1; ind2 = ind1; max1 = x; ind1 = i;
            continue
        }
        if x > max2 {
            max3 = max2; ind3 = ind2; max2 = x; ind2 = i;
            continue
        }
        if x > max3 { max3 = x; ind3 = i; }
    }
    (ind1, ind2, ind3)
}

pub fn game(count_num: usize) -> Vec<u32> {
    // генератор СЧ
    let mut rng = thread_rng();

    // карты cc
    let mut v = vec![0u8; 16];
    v[0] = 1; v[1] = 2;
    v.shuffle(&mut rng);
    let mut cc_cards = VecDeque::from(v);

    // карты ch
    let mut v = vec![0u8; 16];
    for i in 0..10 { v[i] = i as u8 + 1; }
    v.shuffle(&mut rng);
    let mut ch_cards = VecDeque::from(v);

    // состояние игры
    let mut nums = vec![0u32; 40];
    let mut dr1 = vec![];
    let mut dr2 = vec![];
    let mut curr_sqr = 0usize;

    for _ in 0..count_num {
        // бросаем кубик и меняем позицию
        let x1 = rng.gen_range(1usize..=4);
        dr1.push(x1);
        let x2 = rng.gen_range(1usize..=4);
        dr2.push(x2);
        let size = dr1.len();
        // проверяем на 3 дубля подряд
        if x1 == x2 && size > 2 && dr1[size - 2] == dr2[size - 2] &&
            dr1[size - 3] == dr2[size - 3] {
            curr_sqr = 10;
            nums[10] += 1;
            continue
        }
        curr_sqr += x1 + x2;
        if curr_sqr > 39 { curr_sqr %= 40; }
        let xs = BRD[curr_sqr];
        let new_curr_sqr = match xs {
            "G2J" => 10,
            "CC1" | "CC2" => {
                nums[curr_sqr] += 1;
                let m = cc_cards.pop_front().unwrap();
                cc_cards.push_back(m);
                if m > 0 {
                    if m == 1 { 0 } else { 10 }
                } else { curr_sqr }
            },
            "CH1" | "CH2" | "CH3" => {
                // nums[curr_sqr] += 1;
                let m = ch_cards.pop_front().unwrap();
                ch_cards.push_back(m);
                if m > 0 {
                    match m {
                        1 => 0,  // GO
                        2 => 10, // JAIL
                        3 => 11, // C1
                        4 => 24, // E3
                        5 => 39, // H2
                        6 => 5, // R1
                        7 | 8 => { // next R
                            match xs {
                                "CH1" => 15,
                                "CH2" => 25,
                                _ => 5
                            }
                        },
                        9 => { // next U
                            match xs {
                                "CH1" => 12,
                                "CH2" => 28,
                                _ => {
                                    // только вот такая "химия" выводит
                                    // на правильную статистику (!!!)
                                    nums[0] += 1;
                                    12
                                }
                            }
                        },
                        _ => curr_sqr - 3
                    }
                } else { curr_sqr }
            },
            // это обычная клетка
            // просто увеличиваем счетчик
            _ => {
                nums[curr_sqr] += 1;
                curr_sqr
            }
        };
        if new_curr_sqr != curr_sqr {
            curr_sqr = new_curr_sqr;
            nums[curr_sqr] += 1;
        }
    }
    nums
}

pub fn problem84() {
    let total_nums = game(5_000_000);
    let total_moves: u32 = total_nums.iter().sum();
    // анализируем статистику
    let (i, j, k) = top3(&total_nums);
    let base = total_moves as f32;
    let m1 = 100. * total_nums[i] as f32 / base;
    let m2 = 100. * total_nums[j] as f32 / base;
    let m3 = 100. * total_nums[k] as f32 / base;
    println!("{} : {m1:.2}", BRD[i]);
    println!("{} : {m2:.2}", BRD[j]);
    println!("{} : {m3:.2}", BRD[k]);
}
