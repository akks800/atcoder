use proconio::input;

#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::ops::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Struct;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Status {
    neg_time: i64,
    pos: usize,
    silver: usize,
}

//解説を見て作成
//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut silver: usize,
        u_v_cost_time:[(Usize1,Usize1,usize,i64);m],
        xchg_silver_time:[(usize,i64);n],
    }

    let mut e = vec![Vec::new(); n];
    for (u, v, cost, time) in u_v_cost_time {
        e[u].push((v, cost, time));
        e[v].push((u, cost, time));
    }

    struct Dpq {
        dp: Vec<Vec<i64>>,
        q: BinaryHeap<Status>,
    }
    impl Dpq {
        fn push(&mut self, time: i64, pos: usize, silver: usize) {
            if self.dp[pos][silver] > time {
                self.dp[pos][silver] = time;
                self.q.push(Status {
                    neg_time: -time,
                    pos,
                    silver,
                });
            }
        }
    }
    
    let max_silver = 50 * n;
    silver = min(silver, max_silver);
    const MAX_TIME: i64 = 1000_000_000_000_000_000i64;
    let mut dpq = Dpq {
        dp: vec![vec![MAX_TIME; max_silver + 1]; n], // dp[pos][silver] = time
        q: BinaryHeap::new(),
    };

    dpq.push(0, 0, silver);

    let mut ans = vec![MAX_TIME; n];
    ans[0] = 0;
    let mut ct = 1;

    while let Some(s) = dpq.q.pop() {
        //println!("{} {} {}", s.neg_time, s.pos, s.silver);
        if ans[s.pos] == MAX_TIME {
            ans[s.pos] = -s.neg_time;
            ct += 1;
            if ct == n {
                break;
            }
        }

        // 移動した場合
        for (pos, cost, time) in &e[s.pos] {
            if *cost <= s.silver {
                dpq.push(-s.neg_time + *time, *pos, s.silver - *cost);
            }
        }

        // 両替した場合
        let new_time = -s.neg_time + xchg_silver_time[s.pos].1;
        let new_silver = min(s.silver + xchg_silver_time[s.pos].0, max_silver);
        dpq.push(new_time, s.pos, new_silver);
    }

    for i in 1..n {
        println!("{}", ans[i]);
    }
}
