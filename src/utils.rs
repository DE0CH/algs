pub fn ispow2(a: u32) -> bool {
    return a != 0 && (a & (a - 1)) == 0 ;
}

pub fn ceilpow2(a: u32) -> u32 {
    return 1 << if ispow2(a) {31} else {32} - a.leading_zeros();
}