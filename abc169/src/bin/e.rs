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
        n:usize,
        mut v:[(i64,i64);n],
    }
    let mut vl:Vec<i64> = v.iter().map(|x| x.0).collect();
    let mut vh:Vec<i64> = v.iter().map(|x| x.1).collect();
    vl.sort_unstable();
    vh.sort_unstable_by_key(|x|-x);

    /*
       Nが奇数の時 p=(N-1)/2
         最小値は、全てのXがAの場合 vl[p] 
         最大値は、全てのXがBの場合 vh[p]

       Nが偶数の時 p1=N/2-1 p1=N/2
         最小値は (vl[p1]+vl[p2])/2 最大値は (vh[p1]+vh[p2])/2
    */
    let ans;
    if n % 2 == 1 {
        let p = (n-1)/2;
        ans = vh[p]-vl[p]+1;
    } else {
        let p1 = n/2-1;
        let p2 = n/2;
        ans = (vh[p1]+vh[p2]) - (vl[p1]+vl[p2]) + 1;
    }
    println!("{}",ans);

}
