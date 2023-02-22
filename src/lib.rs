pub mod fft;
pub mod matrixpow;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::vec;

    use modular::{Modular, Modulo};

    use crate::fft::multiply_polynomials;
    use crate::fft::pow_polynomial;
    use crate::fft::DEFAULT_NFFT;
    use crate::matrixpow::Matrix;
    use crate::utils::{self, ispow2};

    #[test]
    fn test_ispow2() {
        assert!(ispow2(2));
        assert!(ispow2(4));
        assert!(ispow2(128));
        assert!(!ispow2(100));
    }

    #[test]
    fn test_ceilpow2() {
        assert_eq!(utils::ceilpow2(2), 2);
        assert_eq!(utils::ceilpow2(3), 4);
        assert_eq!(utils::ceilpow2(100), 128);
        assert_eq!(utils::ceilpow2(128), 128);
    }

    #[test]
    fn test_polynomial_mul() {
        let a: Vec<Modulo> = vec![1, 2, 3]
            .into_iter()
            .map(|x| x.to_modulo(DEFAULT_NFFT.r#mod))
            .collect();
        let b: Vec<Modulo> = [4, 59, 6]
            .into_iter()
            .map(|x| x.to_modulo(DEFAULT_NFFT.r#mod))
            .collect();
        let p = multiply_polynomials(a, b).unwrap();
        let ans = vec![4, 67, 136, 189, 18, 0, 0, 0];
        let ans: Vec<Modulo> = ans
            .into_iter()
            .map(|x| x.to_modulo(DEFAULT_NFFT.r#mod))
            .collect();
        assert_eq!(p, ans);
    }

    #[test]
    fn test_polynomial_pow() {
        let a: Vec<Modulo> = vec![1, 2, 3, 4]
            .into_iter()
            .map(|x| x.to_modulo(DEFAULT_NFFT.r#mod))
            .collect();
        let p = pow_polynomial(a.clone(), 100, 20).unwrap();
        let ans: Vec<Modulo> = vec![
            1, 200, 20100, 1353400, 2624253, 5697867, 939281, 3657563, 2901960, 6323049, 383577,
            5869879, 1748976, 559398, 4253052, 4362274, 5435125, 6525776, 2383314, 977420,
        ]
        .into_iter()
        .map(|x| x.to_modulo(DEFAULT_NFFT.r#mod))
        .collect();
        assert_eq!(p, ans);
    }

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
