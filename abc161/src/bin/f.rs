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
// Prime
//

#[allow(dead_code)]
struct Primes {
    size: usize,
    v: Vec<u64>,
    primes: Vec<u64>,
}

#[allow(dead_code)]
impl Primes {
    fn new(size: usize) -> Primes {
        let qsize = ((size as f64).sqrt() as usize) + 1;
        let mut v = vec![0; size];
        let mut primes = Vec::new();
        for i in 2..size {
            if v[i] != 0 {
                continue;
            }
            primes.push(i as u64);
            if i <= qsize {
                let mut j = i * i;
                while j < size {
                    v[j] = 1;
                    j += i;
                }
            }
        }
        Primes { size, v, primes }
    }

    // (素数,べき) のVecを返す
    fn factorv(&self, xx: u64) -> Vec<(u64, usize)> {
        let mut x = xx;
        let mut v = Vec::new();
        for &i in &self.primes {
            let mut ct = 0;
            while x % i == 0 {
                x /= i;
                ct += 1;
            }
            if ct != 0 {
                v.push((i, ct));
                if x < i * i {
                    break;
                }
            }
        }
        if x != 1 {
            v.push((x, 1));
        }
        v
    }

    fn divisor(&self, xx: u64) -> Vec<u64> {
        let v = self.factorv(xx);
        let mut div = Vec::new();
        self.divisor2(&mut div, &v, 1);
        div.sort();
        div
    }

    fn divisor2(&self, div: &mut Vec<u64>, v: &[(u64, usize)], val: u64) {
        if v.len() == 0 {
            div.push(val);
        } else {
            let (prime, pow) = v[0];
            self.divisor2(div, &v[1..], val);
            let mut pp = 1;
            for _i in 1..=pow {
                pp *= prime;
                self.divisor2(div, &v[1..], val * pp);
            }
        }
    }
}

//#[proconio::fastout]
fn main() {
    input! {
        n: u64,
    }

    let p = Primes::new(1000001);
    /* n = (a*k+1)*k^b */

    // b == 0
    let v1 = p.divisor(n - 1);

    // b != 0
    let f = |&&x: &&u64| {
        let mut y = n;
        while x != 1 && y % x == 0 {
            y /= x;
        }
        y % x == 1
    };
    let v2 = p.divisor(n);

    let s: BTreeSet<_> = v1.iter().chain(v2.iter().filter(f)).collect();

    println!("{}", s.len() - 1);
}
