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
        v: [f64;n],
    }

    let ma: f64 = v.iter().map(|x: &f64| x.abs()).sum();
    let eu: f64 = (v.iter().map(|x: &f64| x * x).sum::<f64>()).sqrt();
    let ch: usize = v.iter().map(|x: &f64| x.abs() as usize).max().unwrap();

    println!("{}", ma);
    println!("{}", eu);
    println!("{}", ch);
}
