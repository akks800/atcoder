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
        n: u128,
    }

    let mut c3 = 1;
    for a in 1.. {
        let mut c5 = 1;
        c3 *= 3;
        if c3 > 1000_000_000_000_000_000 {
            break;
        }
        for b in 1.. {
            c5 *= 5;
            if c5 > 1000_000_000_000_000_000 {
                break;
            }
            if c3 + c5 == n {
                println!("{} {}", a, b );
                return;
            }
        }
    }


    println!("-1" );
}
