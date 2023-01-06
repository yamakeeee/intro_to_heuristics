use itertools::*;
use num::*;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use std::collections::BTreeSet;
use std::collections::HashSet;

const UINF: usize = 2000000500;
const INF: i32 = 1000000500;
const LINF: i64 = 1223372036854775807;

const M: usize = 26;
const D: usize = 365;

fn calc_score(c: &[i32], s: &[Vec<i32>], t: &[i32]) {
    let mut res = 0;
    let mut last = vec![0; M];
    for i in 0..D {
        last[t[i] as usize] = i + 1;
        res += s[i][t[i] as usize];
        for j in 0..M {
            res -= (i as i32 + 1 - last[j] as i32) * c[j];
        }
        println!("{}", res);
    }
}

fn main() {
    input! {
        _d:usize,
        c:[i32;M],
        s:[[i32;M];D],
        mut t:[i32;D],
    }
    for i in 0..D {
        t[i] -= 1;
    }
    calc_score(&c, &s, &t);
}
