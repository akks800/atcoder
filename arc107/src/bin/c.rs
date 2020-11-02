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

//
// ModInt
//

const M: i64 = 998244353;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
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
    #[allow(dead_code)]
    fn new_raw(v: i64) -> ModInt {
        ModInt { val: v }
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
        self.mul_assign(ModInt::new(other));
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

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize;n];n],
    }

    // 行の操作と列の操作は互いに影響を与えない。
    // 行a-bが交換できて行b-cも交換できるなら、行a-cも交換できる

    let mut d1 = DSU::new(n);
    let mut d2 = DSU::new(n);
    for i in 0..n - 1 {
        for j in i + 1..n {
            if (0..n).all(|x| a[i][x] + a[j][x] <= k) {
                d1.unify(i, j);
            }
            if (0..n).all(|x| a[x][i] + a[x][j] <= k) {
                d2.unify(i, j);
            }
        }
    }

    let mut c1 = vec![0; n];
    let mut c2 = vec![0; n];
    for i in 0..n {
        c1[d1.find(i)] += 1;
        c2[d2.find(i)] += 1;
    }

    let comb = Comb::new(n + 1);
    let mut ans1 = M1;
    let mut ans2 = M1;
    for i in 0..n {
        ans1 *= comb.fac[c1[i]];
        ans2 *= comb.fac[c2[i]];
    }

    println!("{}", ans1 * ans2);
}
