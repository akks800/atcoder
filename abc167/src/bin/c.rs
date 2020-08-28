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
        x: usize,
        c:[(usize,[usize;m]);n],
    }

    let mut v = Vec::new();
    for bitpat in 0..(1<<n) {
        let mut rikai = vec![0;m];
        let mut cost = 0;
        for i in 0..n {
            if (1<<i) & bitpat != 0 {
                for j in 0..m {
                    rikai[j] += c[i].1[j];
                }
                cost += c[i].0;
            }
        }
        let mut complete = true;
        for j in 0..m {
            if rikai[j] < x {
                complete = false;
                break;
            }
        }
        if complete {
            v.push(cost);
        }
    }

    if let Some(ans) = v.iter().min() {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }

}

