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
        m:usize,
        v:[(Usize1,Usize1);m],
    }
    let mut e = vec![Vec::new();n];
    for (a,b) in &v {
        e[*a].push(*b);
        e[*b].push(*a);
    }

    let mut room_mark:Vec<Option<usize>> = vec![None;n];

    let mut q = VecDeque::new();
    q.push_back(0);
    room_mark[0] = Some(0);

    while let Some(r) = q.pop_front() {
        for next_room in &e[r] {
            if room_mark[*next_room] == None {
                q.push_back(*next_room);
                room_mark[*next_room] = Some(r);
            }
        }
    }

    for i in 1..n {
        if room_mark[i] == None {
            println!("No");
            return;
        }
    }
    println!("Yes");
    for i in 1..n {
        println!("{}", room_mark[i].unwrap()+1 );
    }

}
