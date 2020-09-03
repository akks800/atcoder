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
// ModInt
//

const M: i64 = 1000_000_007;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ModInt {
    val: i64,
}
#[allow(dead_code)]
const M0: ModInt = ModInt { val: 0 };
const M1: ModInt = ModInt { val: 1 };
//const M_ERROR: ModInt = ModInt { val: -1 };

impl ModInt {
    fn new(v: i64) -> ModInt {
        ModInt {
            val: ModInt::val_from_i64(v),
        }
    }
    fn val_from_i64(v: i64) -> i64 {
        let t = v % M;
        if t < 0 {
            t + M
        } else {
            t
        }
    }
    fn val_from_addval(t: i64) -> i64 {
        if t >= M {
            t - M
        } else {
            t
        }
    }

    fn pow(self, exp: i64) -> ModInt {
        if exp == 0 {
            M1
        } else {
            let x = self.pow(exp / 2);
            if exp % 2 == 0 {
                x * x
            } else {
                self * x * x
            }
        }
    }

    fn inv(self) -> ModInt {
        self.pow(M - 2)
    }
}

impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, rhs: ModInt) -> ModInt {
        ModInt {
            val: ModInt::val_from_addval(self.val + rhs.val),
        }
    }
}
impl std::ops::Add<i64> for ModInt {
    type Output = ModInt;
    fn add(self, rhs: i64) -> ModInt {
        ModInt {
            val: ModInt::val_from_i64(self.val + rhs),
        }
    }
}
impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: ModInt) {
        self.val = ModInt::val_from_addval(self.val + other.val);
    }
}
impl std::ops::AddAssign<i64> for ModInt {
    fn add_assign(&mut self, other: i64) {
        self.val = ModInt::val_from_i64(self.val + other);
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: ModInt) -> ModInt {
        ModInt {
            val: ModInt::val_from_addval(self.val - rhs.val + M),
        }
    }
}
impl std::ops::Sub<i64> for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: i64) -> ModInt {
        self - ModInt::new(rhs)
    }
}
impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: ModInt) {
        self.val = ModInt::val_from_addval(self.val - other.val + M);
    }
}
impl std::ops::SubAssign<i64> for ModInt {
    fn sub_assign(&mut self, other: i64) {
        self.val = ModInt::val_from_i64(self.val - other);
    }
}

impl std::ops::Neg for ModInt {
    type Output = ModInt;
    fn neg(self) -> ModInt {
        ModInt {
            val: if self.val == 0 { 0 } else { M - self.val },
        }
    }
}

impl std::ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: ModInt) -> ModInt {
        ModInt {
            val: self.val * rhs.val % M,
        }
    }
}
impl std::ops::Mul<i64> for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: i64) -> ModInt {
        self * ModInt::new(rhs)
    }
}
impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: ModInt) {
        self.val = self.val * other.val % M;
    }
}
impl std::ops::MulAssign<i64> for ModInt {
    fn mul_assign(&mut self, other: i64) {
        self.val = self.val * (other % M) % M;
    }
}

impl std::ops::Div for ModInt {
    type Output = ModInt;
    fn div(self, rhs: ModInt) -> ModInt {
        self * rhs.inv()
    }
}
impl std::ops::Div<i64> for ModInt {
    type Output = ModInt;
    fn div(self, rhs: i64) -> ModInt {
        self / ModInt::new(rhs)
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // '
        // '
        write!(f, "{}", self.val)
    }
}

//
// nCr
//
#[derive(Debug, Clone)]
struct Comb {
    fac: Vec<ModInt>,
    finv: Vec<ModInt>,
    inv: Vec<ModInt>,
}

impl Comb {
    fn new(size: usize) -> Comb {
        let mut fac = vec![M1; size];
        let mut finv = vec![M1; size];
        let mut inv = vec![M1; size];

        for i in 2..size {
            fac[i] = fac[i - 1] * (i as i64);
            inv[i] = -inv[M as usize % i] * (M / (i as i64));
            finv[i] = finv[i - 1] * inv[i];
        }

        return Comb {
            fac: fac,
            finv: finv,
            inv: inv,
        };
    }
}
trait CombC<T> {
    fn c(&self, n: T, k: T) -> ModInt;
    fn p(&self, n: T, k: T) -> ModInt;
}

impl CombC<usize> for Comb {
    fn c(&self, n: usize, k: usize) -> ModInt {
        if n < k {
            M0
        } else {
            self.fac[n] * self.finv[k] * self.finv[n - k]
        }
    }
    fn p(&self, n: usize, k: usize) -> ModInt {
        if n < k {
            M0
        } else {
            self.fac[n] * self.finv[n - k]
        }
    }
}

impl CombC<i64> for Comb {
    fn c(&self, n: i64, k: i64) -> ModInt {
        if n < 0 || k < 0 || n < k {
            M0
        } else {
            self.c(n as usize, k as usize)
        }
    }
    fn p(&self, n: i64, k: i64) -> ModInt {
        if n < 0 || k < 0 || n < k {
            M0
        } else {
            self.p(n as usize, k as usize)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    p: ModInt,
    n: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
    p: ModInt,
    n: usize,
}

// dfs1: 辺の p, n を求める
fn dfs1(
    node: usize,
    parent: usize,
    edge_id: usize,
    v: &Vec<Vec<(usize, usize)>>,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
    comb: &Comb,
) -> (usize, ModInt) {
    if edges[edge_id].n == 0 {
        edges[edge_id] = if nodes[node].n != 0 && edges[edge_id ^ 1].n != 0 {
            let rn = edges[edge_id ^ 1].n;
            Edge {
                n: nodes.len() - rn,
                p: nodes[node].p / (comb.c(nodes.len() - 1, rn) * edges[edge_id ^ 1].p),
            }
        } else {
            let mut node_n = 0;
            let mut node_p = M1;
            let mut div = M1;
            for &(next_node, edge_id) in &v[node] {
                if next_node != parent {
                    let (n, p) = dfs1(next_node, node, edge_id, v, nodes, edges, comb);
                    node_n += n;
                    node_p *= p;
                    div *= comb.fac[n];
                }
            }
            Edge {
                n: node_n + 1,
                p: node_p * comb.fac[node_n] / div,
            }
        }
    }
    (edges[edge_id].n, edges[edge_id].p)
}

fn dfs2(
    node: usize,
    parent: usize,
    v: &Vec<Vec<(usize, usize)>>,
    edges: &Vec<Edge>,
    vv: &mut Vec<usize>,
) {
    vv.push(node);
    for &(next_node, _edge_id) in &v[node] {
        if next_node != parent {
            dfs2(next_node, node, v, edges, vv);
        }
    }
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        ab: [(Usize1,Usize1);n-1],
    }

    /*
     * 枝iの ノード数ni 塗り方pi
     * ノード x の下に 枝 i,j,k がある
     *   nx = 1+ni+nj+nk
     *   px = (ni+nj+nk)!/(ni!nj!nk!) *pi*pj*pk
     */

    let comb = Comb::new(n + 1);
    let mut nodes = vec![Node { p: M0, n: 0 }; n];
    let mut edges = Vec::with_capacity(n * 2);
    let mut v = vec![Vec::new(); n];
    let e = Edge { p: M0, n: 0 };
    for x in ab {
        v[x.0].push((x.1, edges.len()));
        edges.push(e);
        v[x.1].push((x.0, edges.len()));
        edges.push(e);
    }
    let dummy_edge_id = edges.len();
    edges.push(e); // dummy

    {
        let mut vv = Vec::with_capacity(n);
        dfs2(0, n, &v, &edges, &mut vv);
        for i in vv {
            dfs1(i, n, dummy_edge_id, &v, &mut nodes, &mut edges, &comb);
            nodes[i].n = edges[dummy_edge_id].n;
            nodes[i].p = edges[dummy_edge_id].p;
            edges[dummy_edge_id].n = 0;
        }
    }

    for i in 0..n {
        println!("{}", nodes[i].p);
    }
}
