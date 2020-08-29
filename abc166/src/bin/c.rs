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
        m: usize,
        h: [usize;n],
        ab:[(Usize1,Usize1);m],
    }

    let mut v = vec![Vec::new();n];
    for x in ab {
        v[x.0].push(x.1);
        v[x.1].push(x.0);
    }

    let mut ans = 0;
    for i in 0..n {
        if v[i].iter().all(|&j|{h[i] > h[j]}) {
            ans += 1;
        }
    }
    println!("{}", ans);
}

