#![feature(iter_intersperse)]
use algs::fft::{nfft, DEFAULT_NFFT};
use algs::utils::ceilpow2;
use modular::{Modular, Modulo};
use std::env;
use std::iter::once;

fn main() {
    let mut a: Vec<Modulo> = env::args()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap().to_modulo(DEFAULT_NFFT.MOD))
        .collect();
    a.extend(
        once(0.to_modulo(DEFAULT_NFFT.MOD)).cycle().take(
            (ceilpow2(a.len().try_into().unwrap())
                - <usize as TryInto<u32>>::try_into(a.len()).unwrap())
            .try_into()
            .unwrap(),
        ),
    );
    let y = nfft(a.clone(), true).unwrap();
    println!(
        "{:?}",
        y.into_iter().map(|x| x.remainder()).collect::<Vec<i32>>()
    );
}
