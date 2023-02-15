use modinverse::modinverse;
use modular::{Modular, Modulo};
use std::borrow::{BorrowMut, Borrow};
use std::fmt::Display;
use std::{error::Error, vec::Vec};

use super::utils::ispow2;
pub struct NFFT {
    pub MOD: u32,
    pub ROOT: u32,
    pub ROOT_1: u32,
    pub ROOT_PW: u32,
    pub i_2: u32
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
    pub fn new(MOD: u32, ROOT: u32, ROOT_1: u32, ROOT_PW: u32) -> Self{
        NFFT {
            MOD: MOD,
            ROOT: ROOT,
            ROOT_1: ROOT_1,
            ROOT_PW: ROOT_PW,
            i_2: modinverse(2i32, TryInto::<i32>::try_into(MOD).unwrap()).unwrap().try_into().unwrap()
        }
    }
    pub fn nfft(&self, mut a: Vec<Modulo>, invert: bool) -> Result<Vec<Modulo>, InvalidArgumentError> {
        let i_2: i32 = self.i_2.try_into().unwrap();
        let i_2 = i_2.to_modulo(self.MOD);
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
        let wlen: i32 = (if invert { self.ROOT_1 } else { self.ROOT })
            .try_into()
            .unwrap();
        let mut wlen = wlen.to_modulo(self.MOD);
        let mut i: u32 = n.try_into().unwrap();
        while i < self.ROOT_PW {
            wlen = wlen * wlen;
            i <<= 1;
        }

        let mut w = 1i32.to_modulo(self.MOD);
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
}

pub static DEFAULT_NFFT: NFFT = NFFT {
    MOD: 7340033,
    ROOT: 5,
    ROOT_1: 4404020,
    ROOT_PW: 1 << 20,
    i_2: 3670017
};

pub fn nfft(a: Vec<Modulo>, invert: bool) -> Result<Vec<Modulo>, InvalidArgumentError> {
    DEFAULT_NFFT.nfft(a, invert)
}
