pub mod fft;
pub mod utils;

#[cfg(test)]
mod tests {
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
}
