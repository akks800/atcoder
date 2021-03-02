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

fn f(mut x: usize) -> usize {
    let mut v = Vec::new();
    while x != 0 {
        v.push(x % 10);
        x /= 10;
    }
    v.sort();

    let mut ans = 0;
    for i in 0..v.len() {
        ans *= 10;
        ans += v[v.len() - 1 - i];
        ans -= v[i];
    }
    ans
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut memo = BTreeMap::new();
    let mut a = n;
    for _i in 1..=k {
        if let Some(&val) = memo.get(&a) {
            a = val;
        } else {
            let val = f(a);
            memo.insert(a, val);
            a = val;
        }
    }

    println!("{}", a);
}
