use proconio::input;

#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::ops::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Struct;

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        c: i32,
        s: Bytes,
    }

    /*
        d0[i] = (j,k) : today + i 日目以前に、j日働ける。最後に働いたのは today+k 日目
        d1[i] = (j,k) : today + i 日目以降に、j日働ける。次に働くのは、遅くとも today+k日目
    */

    let mut d0 = vec![(0usize, 0); n];
    let mut d1 = vec![(0usize, 0); n];

    {
        let mut last_work_i = -c - 1;
        let mut ct = 0;
        for i in 0..n {
            if last_work_i + c < i as i32 && s[i] == b'o' {
                ct += 1;
                last_work_i = i as i32;
            }
            d0[i] = (ct, last_work_i);
        }
    }

    {
        let mut last_work_i = n as i32 + c;
        let mut ct = 0;
        for i in (0..n).rev() {
            if i as i32 + c < last_work_i && s[i] == b'o' {
                ct += 1;
                last_work_i = i as i32;
            }
            d1[i] = (ct, last_work_i);
        }
    }

    for i in 0..n {
        let (work0, last_work) = if i == 0 { (0, -c - 1) } else { d0[i - 1] };
        let (work1, next_work) = if i == n - 1 {
            (0, n as i32 + c)
        } else {
            d1[i + 1]
        };

        if work0 + work1 < k || work0 + work1 == k && last_work + c >= next_work {
            println!("{}", i + 1);
        }
    }
}
