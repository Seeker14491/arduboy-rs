use crate::c::types::*;

extern "C" {
    #[link_name = "arduino_random_between"]
    fn arduino_random_between_raw(min: c_long, max: c_long) -> c_long;

    #[link_name = "arduino_random_less_than"]
    fn arduino_random_less_than_raw(max: c_long) -> c_long;
}

pub unsafe fn random_between(min: i32, max: i32) -> i32 {
    arduino_random_between_raw(min, max)
}

pub unsafe fn random_less_than(max: i32) -> i32 {
    arduino_random_less_than_raw(max)
}

// Translated from http://xoshiro.di.unimi.it/xoroshiro64star.c

//pub(crate) static mut STATE: [u32; 2] = [0, 1];
//
//fn rotl(x: u32, k: u32) -> u32 {
//    (x << k) | (x >> (32 - k))
//}
//
//pub(crate) fn next() -> u32 {
//    unsafe {
//        let s0 = STATE[0];
//        let mut s1 = STATE[1];
//        let result = s0 * 0x9E3779BB;
//
//        s1 ^= s0;
//        STATE[0] = rotl(s0, 26) ^ s1 ^ (s1 << 9);
//        STATE[1] = rotl(s1, 13);
//
//        result
//    }
//}
