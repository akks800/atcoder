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
        s: [usize;n],
        t: [usize;n],
        u: [u64;n],
        v: [u64;n],
    }

    let mut ans = vec![vec![0u64; n]; n];
    for bit in 0..64 {
        let mut a: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];
        let m = 1u64 << bit;

        let mut row_have_0 = Vec::new();
        let mut row_have_1 = Vec::new();
        let mut col_have_0 = Vec::new();
        let mut col_have_1 = Vec::new();
        let mut row_all_0 = Vec::new();
        let mut row_all_1 = Vec::new();
        let mut col_all_0 = Vec::new();
        let mut col_all_1 = Vec::new();
        let mut row_any = Vec::new();
        let mut col_any = Vec::new();

        for i in 0..n {
            if s[i] == 0 {
                if u[i] & m == 0 {
                    row_have_0.push(i);
                } else {
                    row_all_1.push(i);
                }
            } else {
                if u[i] & m == 0 {
                    row_all_0.push(i);
                } else {
                    row_have_1.push(i);
                }
            }

            if t[i] == 0 {
                if v[i] & m == 0 {
                    col_have_0.push(i);
                } else {
                    col_all_1.push(i);
                }
            } else {
                if v[i] & m == 0 {
                    col_all_0.push(i);
                } else {
                    col_have_1.push(i);
                }
            }
        }

        if row_all_0.len() != 0 && col_all_1.len() != 0
            || row_all_1.len() != 0 && col_all_0.len() != 0
        {
            println!("{}", -1);
            return;
        }

        for &i in &row_all_0 {
            for j in 0..n {
                a[i][j] = Some(0);
            }
        }
        for &i in &row_all_1 {
            for j in 0..n {
                a[i][j] = Some(1);
            }
        }
        for &i in &col_all_0 {
            for j in 0..n {
                a[j][i] = Some(0);
            }
        }
        for &i in &col_all_1 {
            for j in 0..n {
                a[j][i] = Some(1);
            }
        }

        //println!("{:?}", a);

        if row_all_0.len() != 0 {
            col_any.append(&mut col_have_0);
        }
        if row_all_1.len() != 0 {
            col_any.append(&mut col_have_1);
        }
        if col_all_0.len() != 0 {
            row_any.append(&mut row_have_0);
        }
        if col_all_1.len() != 0 {
            row_any.append(&mut row_have_1);
        }

        for _k in 0..2 {
            if col_any.len() != 0 {
                for &i in &row_have_0 {
                    for &j in &col_any {
                        a[i][j] = Some(0);
                    }
                }
                row_any.append(&mut row_have_0);
                for &i in &row_have_1 {
                    for &j in &col_any {
                        a[i][j] = Some(1);
                    }
                }
                row_any.append(&mut row_have_1);
            }
            if row_any.len() != 0 {
                for &i in &col_have_0 {
                    for &j in &row_any {
                        a[j][i] = Some(0);
                    }
                }
                col_any.append(&mut col_have_0);
                for &i in &col_have_1 {
                    for &j in &row_any {
                        a[j][i] = Some(1);
                    }
                }
                col_any.append(&mut col_have_1);
            }

            if row_have_0.len() != 0 && col_have_0.len() != 0 {
                let i = row_have_0.pop().unwrap();
                let j = col_have_0.pop().unwrap();
                a[i][j] = Some(0);
                row_any.push(i);
                col_any.push(j);
            }
            if row_have_1.len() != 0 && col_have_1.len() != 0 {
                let i = row_have_1.pop().unwrap();
                let j = col_have_1.pop().unwrap();
                a[i][j] = Some(1);
                row_any.push(i);
                col_any.push(j);
            }

            if row_have_1.len() >= 2 && col_have_0.len() >= 2 {
                for i in 0..row_have_1.len() {
                    for j in 0..col_have_0.len() {
                        a[row_have_1[i]][col_have_0[j]] = Some((i + j) % 2);
                    }
                }
                row_any.append(&mut row_have_1);
                col_any.append(&mut col_have_0);
            }
            if row_have_0.len() >= 2 && col_have_1.len() >= 2 {
                for i in 0..row_have_0.len() {
                    for j in 0..col_have_1.len() {
                        a[row_have_0[i]][col_have_1[j]] = Some((i + j) % 2);
                    }
                }
                row_any.append(&mut row_have_0);
                col_any.append(&mut col_have_1);
            }
        }

        if row_have_0.len() != 0
            || row_have_1.len() != 0
            || col_have_0.len() != 0
            || col_have_1.len() != 0
        {
            println!("{}", -1);
            return;
        }

        for i in 0..n {
            for j in 0..n {
                if a[i][j] == Some(1) {
                    ans[i][j] |= m;
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{} ", ans[i][j]);
        }
        println!("");
    }
}
