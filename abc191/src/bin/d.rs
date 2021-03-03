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

// [l,h) の中でもっとも手前のfalseになる場所を探す. 全てのtrueはfalseより手前にある必要がある.
fn partition_point<F>(l0: i64, h0: i64, f: F) -> i64
where
    F: Fn(i64) -> bool,
{
    let mut l = l0;
    let mut h = h0;

    while h - l > 0 {
        let m = l + (h - l) / 2;
        if f(m) {
            l = m + 1;
        } else {
            h = m;
        }
    }
    return l;
}

//#[proconio::fastout]
fn main() {
    input! {
        x0: f64,
        y0: f64,
        r0: f64,
    }

    let x = (x0 * 10000.0).round() as i64;
    let y = (y0 * 10000.0).round() as i64;
    let r = (r0 * 10000.0).round() as i64;

    let mut ct = 0;
    let xl = ((x - r) as f64 / 10000.0).floor() as i64 * 10000;
    let xh = ((x + r) as f64 / 10000.0).ceil() as i64 * 10000;
    for xi in (xl..=xh).step_by(10000) {
        // 内側と境界上はfalse
        let yl = partition_point(y - r, y, |yi| {
            r * r < (x - xi) * (x - xi) + (y - yi) * (y - yi)
        });

        // 内側と境界上はtrue
        let yh = partition_point(y, y + r + 1, |yi| {
            r * r >= (x - xi) * (x - xi) + (y - yi) * (y - yi)
        });

        // yl..yh の間に 10000で割れる点は何個あるか
        let yil = ((yl as f64 / 10000.0).ceil() * 10000.0) as i64;
        let yih = ((yh as f64 / 10000.0).ceil() * 10000.0) as i64;
        let tmp = yih - yil;
        let is_outside = |xx, yy| r * r < (xx - x) * (xx - x) + (yy - y) * (yy - y);
        if tmp == 0 {
            assert!(is_outside(xi, yih + 10000));
            assert!(is_outside(xi, yih));
            assert!(is_outside(xi, yih - 10000));
        } else {
            assert!(is_outside(xi, yih));
            assert!(!is_outside(xi, yih - 10000));
            assert!(!is_outside(xi, yil));
            assert!(is_outside(xi, yil - 10000));
        }

        ct += tmp / 10000;
    }

    println!("{}", ct);
}
