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
        n: i32,
        m: i32,
    }

    if m < 0 || n - 2 < m {
        if n == 1 && m == 0 {
            println!("{} {}", 1, 12);
        } else {
            println!("-1");
        }
    } else {
        for i in 0..n {
            if i == 0 {
                println!("{} {}", 1, m * 10 + 12);
            } else {
                println!("{} {}", i * 10, i * 10 + 1);
            }
        }
    }
}
