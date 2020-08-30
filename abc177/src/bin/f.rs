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
struct Struct;

//他人の回答を見て作成
//#[proconio::fastout]
fn main() {
    input! {
        h: usize,
        w:usize,
        ab: [(Usize1,Usize1);h],
    }

    const MAX_WALK: usize = 200005;
    let mut m = BTreeMap::new();
    for i in 0..w {
        m.insert(i, 0);
    }

    let mut w_hist = vec![0; MAX_WALK + 1];
    w_hist[0] = w;
    w_hist[MAX_WALK] = 1;

    let mut v = Vec::with_capacity(w);
    let mut ans = 0;

    for i in 0..h {
        let l = ab[i].0;
        let r = ab[i].1 + 1;
        v.clear();
        let mut min_walk = MAX_WALK;
        for (&pos, &walk) in m.range(l..r + 1) {
            min_walk = min(min_walk, walk + (r - pos));
            v.push(pos);
            w_hist[walk] -= 1;
        }
        for pos in &v {
            m.remove(pos);
        }
        if min_walk != MAX_WALK && r < w {
            m.insert(r, min_walk);
            w_hist[min_walk] += 1;
        }

        while w_hist[ans] == 0 {
            ans += 1;
        }

        // for j in 0..w {
        //     if let Some(wa) = m.get(&j) {
        //         print!("{} ", wa);
        //     } else {
        //         print!("x ",);
        //     }
        // }
        // println!("--{}", ans);
        if ans == MAX_WALK {
            println!("{}", -1);
        } else {
            println!("{}", ans + i + 1);
        }
    }
}
