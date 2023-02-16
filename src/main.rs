#![feature(iter_intersperse)]
use algs::fft::{pow_polynomial, DEFAULT_NFFT};

use modular::Modular;

fn main() {
    let a = vec![1, 1, 1, 1, 1, 1]
        .into_iter()
        .map(|x| x.to_modulo(DEFAULT_NFFT.r#mod))
        .collect();
    let ans = pow_polynomial(a, 100000, 1000).unwrap();
    println!(
        "{:?}",
        ans.into_iter().map(|x| x.remainder()).collect::<Vec<i32>>()
    );
}
