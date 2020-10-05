use proconio::input;

#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::ops::*;

#[derive(Clone, Debug, Default)]
struct Solver {
    pos_empty: Vec<usize>,
    pos_a: Vec<usize>,
    pos_b: Vec<usize>,
    pos_to_ab: Vec<(usize, usize)>,
}

impl Solver {
    fn empty_to_a(&mut self, val: usize) {
        let pos = self.pos_empty.pop().unwrap();
        self.pos_to_ab[pos].0 = val;
        self.pos_a.push(pos);
    }
    fn empty_to_b(&mut self, val: usize) {
        let pos = self.pos_empty.pop().unwrap();
        self.pos_to_ab[pos].1 = val;
        self.pos_b.push(pos);
    }

    fn solve(&mut self) {
        input! {
            n: usize,
            a: [usize;n],
            b: [usize;n],
        }
        let mut hista = vec![0; n + 1];
        let mut histb = vec![0; n + 1];
        self.pos_empty = Vec::with_capacity(n);
        self.pos_a = Vec::with_capacity(n);
        self.pos_b = Vec::with_capacity(n);
        self.pos_to_ab = vec![(0, 0); n];

        for i in 0..n {
            hista[a[i]] += 1;
            histb[b[i]] += 1;
            self.pos_empty.push(i);
        }
        for i in 1..=n {
            if hista[i] + histb[i] > n {
                println!("No");
                return;
            }
            if hista[i] + histb[i] <= self.pos_empty.len() {
                // emptyに充分な空きがある
                for _ in 0..hista[i] {
                    self.empty_to_a(i);
                }
                for _ in 0..histb[i] {
                    self.empty_to_b(i);
                }
            } else {
                // emptyをa,bにいくつ割り当てるか？
                let mut eb = self.pos_empty.len(); // emtpyを全部bに割り当てるとすると?
                if eb > histb[i] {
                    // 余る
                    eb = histb[i];
                }
                if hista[i] > (self.pos_empty.len() - eb) + self.pos_b.len() {
                    // a側の割り当てが足りない
                    eb = self.pos_empty.len() + self.pos_b.len() - hista[i];
                }
                let ea = self.pos_empty.len() - eb;

                for _ in 0..hista[i] - ea {
                    let pos = self.pos_b.pop().unwrap();
                    self.pos_to_ab[pos].0 = i;
                }
                for _ in 0..histb[i] - eb {
                    let pos = self.pos_a.pop().unwrap();
                    self.pos_to_ab[pos].1 = i;
                }
                for _ in 0..eb {
                    self.empty_to_b(i);
                }
                for _ in 0..ea {
                    self.empty_to_a(i);
                }
            }
        }
        /*
        8
        1 1 1 1 1 2 2 2
        1 1 2 2 2 2 4 4
        */
        println!("Yes");
        self.pos_to_ab.sort();
        for &ab in &self.pos_to_ab {
            print!("{} ", ab.1);
        }
        println!("");
    }
}

//#[proconio::fastout]
fn main() {
    let mut s: Solver = Default::default();
    s.solve();
}
