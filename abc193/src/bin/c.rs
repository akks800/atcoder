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
    }

    // x = a^b && 2<=a && 2<=b && x <= n となるxをここに入れる
    let mut s = BTreeSet::new();

    for a in 2.. {
        if a * a > n {
            break;
        }
        for b in 2.. {
            let sab = a.checked_pow(b);
            if let Some(ab) = sab {
                if ab <= n {
                    s.insert(ab);
                }
            } else {
                break;
            }
        }
    }

    println!("{}", n - s.len());
}
