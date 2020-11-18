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

#[derive(Debug, Clone)]
struct DSU {
    v: Vec<usize>,
    ct: Vec<usize>,
    n: usize,
}

#[allow(dead_code)]
impl DSU {
    fn new(size: usize) -> DSU {
        let mut v = vec![0; size];
        for i in 0..size {
            v[i] = i;
        }
        DSU {
            v: v,
            ct: vec![1; size],
            n: size,
        }
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
            self.ct[xx] += self.ct[yy];
            self.v[yy] = xx;
        }
    }
    fn get_size(&mut self, x: usize) -> usize {
        let xx = self.find(x);
        self.ct[xx]
    }
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        c: [Usize1;n],
        mut qu: [(usize,Usize1,Usize1);q],
    }

    let mut d = DSU::new(n);
    let mut v = vec![Vec::new(); n]; // v[i]: ノードiの子ノード
    let mut q2x = vec![0; q];
    let mut q2len = vec![0; q];

    for qid in 0..q {
        if qu[qid].0 == 1 {
            let a = d.find(qu[qid].1);
            let b = d.find(qu[qid].2);
            if a != b {
                v[a].push(b);
                d.unify(a, b);
            }
        } else {
            let x = qu[qid].1;
            q2x[qid] = d.find(x);
            q2len[qid] = d.get_size(x);
        }
    }

    // i2pos
    let mut pos = 0;
    let mut i2pos = vec![0; n];
    fn dsu(i: usize, v: &Vec<Vec<usize>>, mut pos: usize, i2pos: &mut Vec<usize>) -> usize {
        i2pos[i] = pos;
        pos += 1;
        for &j in &v[i] {
            pos = dsu(j, v, pos, i2pos);
        }
        pos
    }
    for i in 0..n {
        if d.find(i) == i {
            pos = dsu(i, &v, pos, &mut i2pos);
        }
    }

    // posq
    #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct PosQ {
        pos: usize,
        qid: usize,
        color: usize,
        is_add: bool,
    }
    let mut posq = Vec::with_capacity(q * 2);
    for qid in 0..q {
        if qu[qid].0 == 2 {
            posq.push(PosQ {
                pos: i2pos[q2x[qid]],
                qid: qid,
                color: qu[qid].2,
                is_add: false,
            });
            posq.push(PosQ {
                pos: i2pos[q2x[qid]] + q2len[qid],
                qid: qid,
                color: qu[qid].2,
                is_add: true,
            });
        }
    }
    posq.sort_unstable();
    posq.push(PosQ {
        pos: n + 1,
        qid: 0,
        color: 0,
        is_add: false,
    });

    // pos2color
    let mut pos2color = vec![0; n];
    for i in 0..n {
        pos2color[i2pos[i]] = c[i];
    }


    // posの順に更新
    let mut ans = vec![0; q];
    let mut color_ct = vec![0; n];
    let mut j = 0;
    for pos in 0..=n {
        while posq[j].pos == pos {
            if posq[j].is_add {
                ans[posq[j].qid] += color_ct[posq[j].color];
            } else {
                ans[posq[j].qid] -= color_ct[posq[j].color];
            }
            j += 1;
        }
        if pos != n {
            color_ct[pos2color[pos]] += 1;
        }
    }

    //
    for qid in 0..q {
        if qu[qid].0 == 2 {
            println!("{}", ans[qid]);
        }
    }
}
