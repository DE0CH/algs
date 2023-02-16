#![feature(iter_intersperse)]
use algs::fft::{nfft, DEFAULT_NFFT, pow_polynomial};
use algs::utils::ceilpow2;
use modular::{Modular, Modulo};
use std::env;
use std::iter::once;

fn main() {
    let a = vec![1, 1, 1, 1, 1, 1].into_iter().map(|x| x.to_modulo(DEFAULT_NFFT.r#mod)).collect();
    let ans = pow_polynomial(a, 100000, 1000).unwrap(); 
    println!("{:?}", ans.into_iter().map(|x| x.remainder()).collect::<Vec<i32>>());
}
