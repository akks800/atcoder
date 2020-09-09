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

const M: i64 = 998244353;
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
        write!(f, "{}", self.val)
    }
}

//
// Segment Tree
//

#[derive(Clone)]
struct SegmentTree<T> {
    v: Vec<T>,
    offset: usize,
    comb: fn(T, T) -> T,
    unit: T,
}

#[allow(dead_code)]
impl<T: Copy> SegmentTree<T> {
    fn new(n: usize, unit: T, comb: fn(T, T) -> T, vv: Option<&Vec<T>>) -> SegmentTree<T> {
        let mut x = 1;
        while x < n {
            x *= 2;
        }

        let mut v = vec![unit; x * 2];
        if let Some(vv) = vv {
            for i in 0..n {
                v[x + i] = vv[i];
            }
            for i in (0..x).rev() {
                v[i] = comb(v[i * 2], v[i * 2 + 1]);
            }
        }

        SegmentTree {
            v: v,
            offset: x,
            comb,
            unit: unit,
        }
    }

    fn update(&mut self, pos: usize, val: T) {
        let mut p = self.offset + pos;
        self.v[p] = val;
        while p != 1 {
            p /= 2;
            self.v[p] = (self.comb)(self.v[p * 2], self.v[p * 2 + 1]);
        }
    }

    fn query_range(&self, begin: usize, end: usize) -> T {
        self.query_range_sub(begin, end, 1, 0, self.offset)
    }

    // v[node] は 区間 l .. r-1 の計算結果を持っている
    fn query_range_sub(&self, begin: usize, end: usize, node: usize, l: usize, r: usize) -> T {
        if end <= l || r <= begin {
            return self.unit.clone();
        } else if begin <= l && r <= end {
            return self.v[node].clone();
        } else {
            let lval = self.query_range_sub(begin, end, node * 2, l, (l + r) / 2);
            let rval = self.query_range_sub(begin, end, node * 2 + 1, (l + r) / 2, r);
            return (self.comb)(lval, rval);
        }
    }
}

// [l,h) の中でもっとも手前のfalseになる場所を探す. 全てのtrueはfalseより手前にある必要がある.
fn partition_point<F>(l0: usize, h0: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut l = l0;
    let mut h = h0;

    while h - l > 0 {
        let m = l + (h - l) / 2;
        if f(m) {
            l = m + 1;
        } else {
            h = m;
        }
    }
    return l;
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        mut xd: [(i32,i32);n],
    }

    xd.sort_unstable();
    // dp[i] : ロボットi以降の、残るロボットの組み合わせの数
    // i を起動した結果 [i+1,j) が起動するなら、
    // dp[i] = iを起動した場合 + iを起動しなかった場合
    //       = dp[j]          + dp[i+1]
    let mut dp = vec![M0; n + 1];
    dp[n] = M1;

    let pos_to_xd: Vec<i32> = xd.iter().map(|p| p.0 + p.1).collect();
    let mut s = SegmentTree::new(n, -1000_000_005, |l, r| max(l, r), Some(&pos_to_xd));
    for i in (0..n).rev() {
        let mut xd_max = xd[i].0 + xd[i].1;
        loop {
            let j = partition_point(i + 1, n, |k| xd[k].0 < xd_max);
            let next_xd_max = s.query_range(i, j);
            if next_xd_max > xd_max {
                xd_max = next_xd_max;
                continue;
            }

            s.update(i, xd_max);
            //println!("{} {}", i, j);
            dp[i] = dp[i + 1] + dp[j];
            break;
        }
    }

    println!("{}", dp[0]);
}
