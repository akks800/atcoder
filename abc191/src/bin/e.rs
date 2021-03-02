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
    cost: i64,
    start: usize,
    end: usize,
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc:[(Usize1,Usize1,i64);m],
    }

    const MAX_COST: i64 = 2000 * 100000 + 1;
    let mut q = BinaryHeap::new();
    let mut min_cost = vec![vec![MAX_COST; n]; n]; // [i][j] : iからjに行くのにかかる時間

    let mut v = vec![Vec::new(); n];
    for (start, end, cost) in abc {
        v[start].push((end, cost));
        q.push(Reverse(S {
            cost,
            start,
            end,
        }));
    }

    while let Some(Reverse(s)) = q.pop() {
        let cost = s.cost;
        let start = s.start;
        let end = s.end;
        if min_cost[start][end] > cost {
            min_cost[start][end] = cost;
            for &(next, addcost) in &v[end] {
                q.push(Reverse(S {
                    cost: cost + addcost,
                    start,
                    end: next,
                }));
            }
        }
    }

    for i in 0..n {
        println!(
            "{}",
            if min_cost[i][i] != MAX_COST {
                min_cost[i][i]
            } else {
                -1
            }
        );
    }
}
