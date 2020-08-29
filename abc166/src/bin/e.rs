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
        a: [usize;n],
    }

    // i-Ai == j+Aj || i+Ai == j-Aj
    let mut p = BTreeMap::new();
    for i in 0..n {
        let k = i+1 + a[i];
        //p.entry(k).and_modify(|x|{*x+=1}).or_insert(1);
        *p.entry(k).or_default() += 1;
    }


    let mut ans = 0u64;
    for i in 0..n {
        if i+1 > a[i] {
            let k = i+1-a[i];
            if let Some(ct) = p.get(&k) {
                ans += ct;
            }
        }
    }

    println!("{}", ans);
}

