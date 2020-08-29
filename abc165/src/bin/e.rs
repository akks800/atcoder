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
    }

    let al = 0;
    let ah = (n - 1) / 2;
    let bl = n / 2 + 1;
    let bh = n - 1;

    let mut v = Vec::new();
    for i in 0..=m / 2 {
        v.push((al + i, ah - i));
        v.push((bl + i, bh - i));
    }

    for ab in v.iter().take(m) {
        println!("{} {}", ab.0 + 1, ab.1 + 1);
        //println!("{} {} {} ", ab.0 + 1, ab.1 + 1, (ab.1-ab.0+n)%n);
    }
}
