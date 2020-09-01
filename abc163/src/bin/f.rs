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
        c: [Usize1;n],
        ab:[(Usize1,Usize1);n-1],
    }

    let mut v = vec![Vec::new(); n];
    for &p in &ab {
        v[p.0].push(p.1);
        v[p.1].push(p.0);
    }

    struct Ctx {
        c: Vec<usize>,
        color_ct: Vec<Vec<i64>>,
        color_sizes: Vec<Vec<i64>>,
        clock: i64,
    }
    let mut ctx = Ctx {
        c,
        color_ct: vec![vec![0; 1]; n], // 他の部分木内で過ごした時間
        color_sizes: vec![Vec::new(); n],
        clock: 0,
    };

    /*
       ノード数Nの木の、全てのパスの数 = N*(N+1)/2
       色kを通るパスの数 = 全てのパスの数 - 色kを通らないパスの数

       木を、色kを含まない複数の部分木(ノード数 s1, s2, ... sm)に分ける。
       色kを通らないパスの数 = Σ si*(si+1)/2

       部分木のサイズをどうやって求めるか？
       dfsで
       部分木のサイズ = 子ノードに入ってから出るまでの時間 - Σ 子ノードの先の他の部分木に入ってから出るまでの時間
    */
    impl Ctx {
        fn dfs(&mut self, node: usize, parent: usize, v: &Vec<Vec<usize>>) {
            self.clock += 1;
            let color = self.c[node];
            let start_clock = self.clock;

            for &next_node in &v[node] {
                if next_node != parent {
                    let start_clock = self.clock;
                    self.color_ct[color].push(0);
                    self.dfs(next_node, node, v);
                    let ct = self.color_ct[color].pop().unwrap();
                    self.color_sizes[color].push(self.clock - start_clock - ct);
                }
            }

            *self.color_ct[color].last_mut().unwrap() += 1 + self.clock - start_clock;
        }
    }

    ctx.dfs(0, n, &v);
    for i in 0..n {
        ctx.color_sizes[i].push(n as i64 - ctx.color_ct[i][0]);
    }

    for i in 0..n {
        let mut ans = n as i64 * (n + 1) as i64 / 2;
        //print!(" i={} ", i);
        for &size in &ctx.color_sizes[i] {
            let size = size as i64;
            ans -= size * (size + 1) / 2;
            //print!(" s={} ", size);
        }
        println!("{} ", ans);
    }
}
