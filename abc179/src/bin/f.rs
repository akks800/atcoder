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

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        qs: [(usize,usize);q],
    }

    let mut m1 = BTreeMap::new();
    let mut m2 = BTreeMap::new();
    m1.insert(n, n);
    m2.insert(n, n);

    let mut ans = (n-2) as u64 *(n-2) as u64;
    for (op, x) in qs {
        if op == 1 {
            // get depth
            let (_, &depth) = m1.range(x..).next().unwrap();
            //println!("1 x={} depth={}", x, depth);
            m2.entry(depth).and_modify(|e| *e = min(*e, x)).or_insert(x);
            ans -= depth as u64 - 2;
        } else {
            let (_, &depth) = m2.range(x..).next().unwrap();
            //println!("2 x={} depth={}", x, depth);
            m1.entry(depth).and_modify(|e| *e = min(*e, x)).or_insert(x);
            ans -= depth as u64 - 2;
        }
    }

    println!("{}", ans);
}
