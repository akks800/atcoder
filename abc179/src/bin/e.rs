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
        mut n: usize,
        x: usize,
        m: usize,
    }

    let mut a_to_i = vec![0; m];
    let mut acc = vec![0; m + 1]; // acc[j] = Σai  1<=i<=j

    let mut i = 1;
    let mut a = x;
    let mut asum = x as u64;
    acc[i] = asum;
    a_to_i[a] = i;

    while i < n {
        let next_a = (a as u64 * a as u64 % m as u64) as usize;
        if a_to_i[next_a] != 0 {
            /*
               1  2            before_loop // i_before_loop=2
                     3  4  5   loop // period=3
                     6  7  8   loop // complete_loop_count=2
                     9  10     after_loop // after_loop_len=2
                now i = 5
            */
            let i_before_loop = a_to_i[next_a] - 1;
            let period = i - i_before_loop;
            let complete_loop_count = (n - i_before_loop) / period;
            let after_loop_len = n - i_before_loop - complete_loop_count * period;

            let sum_before_loop = acc[i_before_loop];
            let sum_in_loop = (acc[i] - acc[i_before_loop]) * complete_loop_count as u64;
            let sum_after_loop = acc[i_before_loop + after_loop_len] - acc[i_before_loop];
            let ans = sum_before_loop + sum_in_loop + sum_after_loop;

            println!("{}", ans);

            return;
        }

        i += 1;
        a = next_a;
        asum += a as u64;
        acc[i] = asum;
        a_to_i[next_a] = i
    }

    println!("{}", asum);
}
