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
        a: [usize;n],
    }

    let mut hist = vec![0; n]; // hitogram
    let mut b: BTreeSet<usize> = BTreeSet::new();
    for i in 0..=n {
        b.insert(i);
    }

    for i in 0..m {
        let val = a[i];
        if hist[val] == 0 {
            b.remove(&val);
        }
        hist[val] += 1;
    }

    let mut min_mex = n;
    for i in 0..=n - m {
        if i != 0 {
            // remove a[i-1]
            let val = a[i - 1];
            if hist[val] == 1 {
                b.insert(val);
            }
            hist[val] -= 1;
            // add a[i+m-1]
            let val = a[i + m - 1];
            if hist[val] == 0 {
                b.remove(&val);
            }
            hist[val] += 1;
        }
        let &mex = b.range(0..).next().unwrap();
        min_mex = min(min_mex, mex);
    }
    println!("{}", min_mex);
}
