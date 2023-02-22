use modular::{Modular, Modulo};
use std::ops::Mul;

#[derive(Clone, PartialEq, Debug)]
struct Matrix(Vec<Vec<Modulo>>);

impl Matrix {
    fn pow(&self, n: u64) -> Self {
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
    fn mul(&self, rhs: &Self) -> Self {
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

#[cfg(test)]
mod test {
    use modular::Modular;
    use modular::Modulo;

    use crate::matrixpow::Matrix;

    #[test]
    fn test_mul() {
        let p = vec![vec![1, 2], vec![2, 3]];
        let p: Vec<Vec<Modulo>> = p
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.to_modulo(1000000007)).collect())
            .collect();
        let p = Matrix(p);
        let p = p.pow(6);
        let ans = vec![vec![1597, 2584], vec![2584, 4181]];
        let ans: Vec<Vec<Modulo>> = ans
            .into_iter()
            .map(|x| x.into_iter().map(|x| x.to_modulo(1000000007)).collect())
            .collect();
        let ans = Matrix(ans);
        assert_eq!(p, ans);
    }
}
