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

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize;n],
    }

    // dp[i][j]: a[0..i]からいくつか選んで合計がjになる場合の数 × Lの選び方の数
    let mut dp = vec![vec![M0; 3001]; n + 1];

    let mut ans = M0;
    for i in 1..=n {
        let val = a[i - 1];
        // 既にいくつか選んでいて、Aiを足す場合
        for j in val..=s {
            if j == s {
                ans += dp[i - 1][j - val] * (n - i + 1) as i64;
            } else {
                dp[i][j] = dp[i][j] + dp[i - 1][j - val];
            }
        }
        // 既にいくつか選んでいて、Aiを足さない場合
        for j in 0..s {
            dp[i][j] = dp[i][j] + dp[i - 1][j];
        }

        // まだ選んでなくて、Aiを足す場合
        if val == s {
            ans += (i * (n - i + 1)) as i64
        } else {
            dp[i][val] += i as i64;
        }
    }
/*
    for i in 0..=n {
        for j in 0..=s {
            print!("{} ", dp[i][j]);
        }
        println!(" ");
    }
*/

    println!("{}", ans);
}
