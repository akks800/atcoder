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
        s: Bytes,
        nq: usize,
    }

    let mut reversed = false;
    let mut head = VecDeque::new();
    let mut tail = Vec::new();
    for _ in 0..nq {
        input! { t: usize }
        if t == 1 {
            reversed = !reversed;
        } else {
            input! {
                f: usize,
                c: Bytes,
            }
            if f == 1 && !reversed || f == 2 && reversed {
                head.push_front(c[0]);
            } else {
                tail.push(c[0]);
            }
        }
    }

    let it = head.iter().chain(s.iter()).chain(tail.iter());
    let mut s: Vec<u8> = it.map(|&x| x).collect();
    if reversed {
        s.reverse();
    }
    println!("{}", String::from_utf8(s).unwrap());
}
