use modinverse::modinverse;
use modular::{Modular, Modulo};
use std::fmt::Display;
use std::iter::{repeat, zip};
use std::{error::Error, vec::Vec};

use crate::utils::ceilpow2;

use super::utils::ispow2;
pub struct NFFT {
    pub r#mod: u32,
    pub root: u32,
    pub root_1: u32,
    pub root_pw: u32,
    pub i_2: u32,
}

#[derive(Debug)]
pub struct InvalidArgumentError(String);
impl Display for InvalidArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Argument is invalid, the reason is: {}", self.0)
    }
}

impl Error for InvalidArgumentError {}

impl NFFT {
    pub fn new(r#mod: u32, root: u32, root_1: u32, root_pw: u32) -> Self {
        NFFT {
            r#mod,
            root,
            root_1,
            root_pw,
            i_2: modinverse(2i32, TryInto::<i32>::try_into(r#mod).unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
        }
    }
    pub fn nfft(
        &self,
        mut a: Vec<Modulo>,
        invert: bool,
    ) -> Result<Vec<Modulo>, InvalidArgumentError> {
        let i_2: i32 = self.i_2.try_into().unwrap();
        let i_2 = i_2.to_modulo(self.r#mod);
        let n = a.len();
        if !ispow2(n.try_into().unwrap()) {
            return Err(InvalidArgumentError(
                "The length of a has to be a power of 2".to_string(),
            ));
        }
        if n == 1 {
            return Ok(a);
        }

        let mut a0: Vec<Modulo> = Vec::new();
        let mut a1: Vec<Modulo> = Vec::new();
        a0.reserve_exact(n / 2);
        a1.reserve_exact(n / 2);
        for i in 0..n / 2 {
            a0.push(a[i * 2]);
            a1.push(a[i * 2 + 1]);
        }

        let y0 = self.nfft(a0, invert)?;
        let y1 = self.nfft(a1, invert)?;
        let wlen: i32 = (if invert { self.root_1 } else { self.root })
            .try_into()
            .unwrap();
        let mut wlen = wlen.to_modulo(self.r#mod);
        let mut i: u32 = n.try_into().unwrap();
        while i < self.root_pw {
            wlen = wlen * wlen;
            i <<= 1;
        }

        let mut w = 1i32.to_modulo(self.r#mod);
        for i in 0..n / 2 {
            a[i] = y0[i] + w * y1[i];
            a[n / 2 + i] = y0[i] - w * y1[i];
            if invert {
                a[i] = a[i] * i_2;
                a[n / 2 + i] = a[n / 2 + i] * i_2;
            }
            w = w * wlen;
        }
        let y = a;
        Ok(y)
    }

    pub fn multiply_polynomials(
        &self,
        mut a: Vec<Modulo>,
        mut b: Vec<Modulo>,
    ) -> Result<Vec<Modulo>, InvalidArgumentError> {
        let n: usize = ceilpow2((a.len() + b.len()) as u32).try_into().unwrap();
        a.extend(repeat(0.to_modulo(self.r#mod)).take(n - a.len()));
        b.extend(repeat(0.to_modulo(self.r#mod)).take(n - b.len()));
        let a1 = self.nfft(a, false)?;
        let b1 = self.nfft(b, false)?;
        let p1: Vec<Modulo> = zip(a1.into_iter(), b1.into_iter())
            .map(|(x, y)| x * y)
            .collect();
        let p = self.nfft(p1, true)?;
        Ok(p)
    }

    pub fn pow2_polynomial(&self, mut a: Vec<Modulo>) -> Result<Vec<Modulo>, InvalidArgumentError> {
        let n: usize = ceilpow2((a.len() * 2) as u32).try_into().unwrap();
        a.extend(repeat(0.to_modulo(self.r#mod)).take(n - a.len()));
        let a1 = self.nfft(a, false)?;
        let p1: Vec<Modulo> = a1.into_iter().map(|x| x * x).collect();
        let p = self.nfft(p1, true)?;
        Ok(p)
    }

    pub fn pow_polynomial(
        &self,
        a: Vec<Modulo>,
        n: u32,
        k: usize,
    ) -> Result<Vec<Modulo>, InvalidArgumentError> {
        if n == 1 {
            return Ok(a);
        }
        if n % 2 != 0 {
            let mut ans =
                self.multiply_polynomials(a.clone(), self.pow_polynomial(a, n - 1, k)?)?;
            ans.resize(k, 0.to_modulo(self.r#mod));
            Ok(ans)
        } else {
            let p = self.pow_polynomial(a, n / 2, k)?;
            let mut ans = self.pow2_polynomial(p)?;
            ans.resize(k, 0.to_modulo(self.r#mod));
            Ok(ans)
        }
    }
}

pub static DEFAULT_NFFT: NFFT = NFFT {
    r#mod: 7340033,
    root: 5,
    root_1: 4404020,
    root_pw: 1 << 20,
    i_2: 3670017,
};

pub fn nfft(a: Vec<Modulo>, invert: bool) -> Result<Vec<Modulo>, InvalidArgumentError> {
    DEFAULT_NFFT.nfft(a, invert)
}

pub fn multiply_polynomials(
    a: Vec<Modulo>,
    b: Vec<Modulo>,
) -> Result<Vec<Modulo>, InvalidArgumentError> {
    DEFAULT_NFFT.multiply_polynomials(a, b)
}

pub fn pow_polynomial(
    a: Vec<Modulo>,
    n: u32,
    k: usize,
) -> Result<Vec<Modulo>, InvalidArgumentError> {
    DEFAULT_NFFT.pow_polynomial(a, n, k)
}
