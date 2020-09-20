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
struct Solver {}

impl Solver {
    fn solve(&mut self) {
        input! {
            n: usize,
            a: [usize;n],
            mut b: [usize;n],
        }

        b.reverse();
        let mut conflict = None;
        for i in 0..n {
            if a[i] == b[i] {
                conflict = Some((i, a[i]));
                break;
            }
        }
        if let Some((p2, val)) = conflict {
            //         0    p1    p2    p3    p4    n
            // level   00000111111222222111111000000
            let level = |i| if a[i] == val { 1 } else { 0 } + if b[i] == val { 1 } else { 0 };
            let p1 = (0..p2).filter(|&i| level(i) == 0).count();
            let p3 = p2 + (p2..n).filter(|&i| level(i) == 2).count();
            let p4 = p3 + (p3..n).filter(|&i| level(i) == 1).count();

            if (p1 - 0) + (n - p4) < p3 - p2 {
                println!("No");
                return;
            }
            let p = min( p2 + (p1 - 0), p3 );
            &b[0..p].reverse();
            &b[p..n].reverse();
        }

        println!("Yes");
        for i in 0..n {
            print!("{} ", b[i]);
        }
        println!("");
    }
}

//#[proconio::fastout]
fn main() {
    let mut s: Solver = Default::default();
    s.solve();
}
