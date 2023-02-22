use modular::{Modular, Modulo};

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix(pub Vec<Vec<Modulo>>);

impl Matrix {
    pub fn pow(&self, n: u64) -> Self {
        if n <= 1 {
            panic!("n must be larger than 1");
        } else if n == 2 {
            self.mul(&self)
        } else if n % 2 == 0 {
            let p = self.pow(n / 2);
            p.mul(&p)
        } else {
            let p = self.pow(n - 1);
            self.mul(&p)
        }
    }
    pub fn mul(&self, rhs: &Self) -> Self {
        let n = self.0.len();
        let m = rhs.0[0].len();
        let o = rhs.0.len();
        let mut res = vec![vec![0.to_modulo(self.0[0][0].modulus()); n]; m];
        for i in 0..n {
            for j in 0..m {
                for k in 0..o {
                    res[i][j] = res[i][j] + (self.0[i][k] * rhs.0[k][j]);
                }
            }
        }
        Self(res)
    }
}
