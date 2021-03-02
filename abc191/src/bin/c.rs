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
        h: usize,
        w: usize,
        t: [Chars;h],
    }

    let mut ans = 0;
    let f = |c| if c == '#' {1} else {0};
    for x in 0..w-1 {
        for y in 0..h-1 {
            let z = 
              f(t[y][x])+
              f(t[y][x+1])+
              f(t[y+1][x])+
              f(t[y+1][x+1]);
            if z == 1 || z == 3 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
