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


//
// DSU
//

#[derive(Debug, Clone)]
struct DSU {
    v: Vec<usize>,
    n: usize,
}

#[allow(dead_code)]
impl DSU {
    fn new(size: usize) -> DSU {
        let mut v = vec![0; size];
        for i in 0..size {
            v[i] = i;
        }
        DSU { v: v, n: size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.v[x] == x {
            x
        } else {
            self.v[x] = self.find(self.v[x]);
            self.v[x]
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unify(&mut self, x: usize, y: usize) {
        let xx = self.find(x);
        let yy = self.find(y);
        if xx != yy {
            self.v[yy] = xx;
        }
    }
}



//#[proconio::fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }

    let mut d = DSU::new(n);
    for x in &ab {
        d.unify(x.0,x.1);
    }

    let mut group_size = vec![0;n];
    for i in 0..n {
        group_size[d.find(i)] += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, group_size[i]);
    }

    println!("{}", ans);
}
