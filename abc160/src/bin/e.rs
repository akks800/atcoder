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

//#[proconio::fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        p: [usize;a],
        q: [usize;b],
        r: [usize;c],
    }

    let mut v = Vec::with_capacity(a + b + c);
    for x in p {
        v.push((x, b'R'));
    }
    for x in q {
        v.push((x, b'G'));
    }
    for x in r {
        v.push((x, b'W'));
    }

    v.sort_unstable_by_key(|x| 1000_000_000 - x.0);
    let mut red = 0;
    let mut green = 0;
    let mut white = 0;
    let mut ans = 0;
    for i in 0..v.len() {
        match v[i].1 {
            b'R' => {
                if red < x {
                    ans += v[i].0;
                    red += 1;
                }
            }
            b'G' => {
                if green < y {
                    ans += v[i].0;
                    green += 1;
                }
            }
            b'W' => {
                ans += v[i].0;
                white += 1;
            }
            _ => panic!(),
        }
        if red + green + white == x + y {
            break;
        }
    }

    println!("{}", ans);
}
