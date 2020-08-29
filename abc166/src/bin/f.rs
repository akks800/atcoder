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

/**
 * 現在のa,b,c の値
 * 現在の移動方向 a固定
 * 次の移動方向 next固定
 * bcのどちらに1を足すかを返す
 */
fn f( _a:&mut i32, b:&mut i32, c:&mut i32, next:i32 ) -> i32 {
    if *b == 1 && *c == 1 {
        if next == 2 {
            *b += 1;
            *c -= 1;
            1
        } else {
            *b -= 1;
            *c += 1;
            2
        }
    } else {
        if *b < *c {
            *b += 1;
            *c -= 1;
            1
        } else {
            *b -= 1;
            *c += 1;
            2
        }
    }
}


//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        mut a: i32,
        mut b: i32,
        mut c: i32,
        s: [String;n],
    }

    let mut v = Vec::with_capacity(n+1);
    for i in 0..n {
        v.push(
            match s[i].as_str() {
                "AB" => 2,
                "BC" => 0,
                "AC" => 1,
                _=>panic!(),
            },
        );
    }

    v.push( 0 );
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        //println!("{} {} {} {}", a, b, c, v[i]);
        let r = match v[i] {
            0 => f(&mut a,&mut b,&mut c,v[i+1]),
            1 => (f(&mut b,&mut c,&mut a,(v[i+1]+3-1)%3)+1)%3,
            2 => (f(&mut c,&mut a,&mut b,(v[i+1]+3-2)%3)+2)%3,
            _ => panic!(),
        };
        if a < 0 || b < 0 || c < 0 {
            println!("No");
            return;
        }
        ans.push(r);
    }
    println!("Yes");
    for i in 0..n {
        println!("{}", match ans[i] {
            0 => "A",
            1 => "B",
            2 => "C",
            _ => panic!(),
        });
    }
}
