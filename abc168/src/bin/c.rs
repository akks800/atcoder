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
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    let minutes = h*60.0+m;
    let a_angle = minutes/(60.0*12.0) * std::f64::consts::PI * 2.0;
    let b_angle = minutes/60.0 * std::f64::consts::PI * 2.0;
    let angle = a_angle - b_angle;

    let dx = a - b * angle.cos();
    let dy = b * angle.sin();


    println!("{}", (dx*dx+dy*dy).sqrt() );
}

