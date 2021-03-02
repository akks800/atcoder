use proconio::input;

#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::ops::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct S {
    time: i64,
    pos: usize,
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
        abtk:[(Usize1,Usize1,i64,i64);m], // 鉄道
    }

    let mut v = vec![Vec::new(); n]; // v[i] : (j,t,k) 都市 i-j 間の鉄道
    for &p in &abtk {
        v[p.0].push((p.1, p.2, p.3));
        v[p.1].push((p.0, p.2, p.3));
    }

    let mut q = BinaryHeap::new();
    const MAX_COST: i64 = 100000 * 2000000000 + 1;
    let mut best = vec![MAX_COST; n];
    q.push(Reverse(S { time: 0, pos: x }));

    while let Some(Reverse(s)) = q.pop() {
        let time = s.time;
        let pos = s.pos;

        if time < best[pos] {
            best[pos] = time;

            for &(j, t, k) in &v[pos] {
                let next_time = if time % k == 0 {
                    time + t
                } else {
                    time - (time % k) + k + t
                };

                if next_time < best[j] {
                    q.push(Reverse(S {
                        time: next_time,
                        pos: j,
                    }));
                }
            }
        }
    }

    println!("{}", if best[y] == MAX_COST { -1 } else { best[y] });
}
