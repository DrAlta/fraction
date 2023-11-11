

use num::rational::Ratio;

use crate::GenericFraction;

pub trait Consts {

    const E: Self;
    const FRAC_2_PI: Self;
    const FRAC_PI_2: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const FRAC_PI_8: Self;
    const PI: Self;
    const SQRT_2: Self;
    const TAU: Self;

}


impl Consts for GenericFraction<i8> {// 127
    const E: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(106, 39));
    const FRAC_2_PI: Self = Self::TAU;
    const FRAC_PI_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11, 7));
    const FRAC_PI_4: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11, 14));
    const FRAC_PI_6: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11, 21));
    const FRAC_PI_8: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11, 28));
    const PI: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(22, 7));
    const SQRT_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(99, 70));
    const TAU: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(44, 7));
}
impl Consts for GenericFraction<u8> { //255
    const E: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(193, 71));
    const FRAC_2_PI: Self = Self::TAU;
    const FRAC_PI_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11, 7));
    const FRAC_PI_4: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11, 14));
    const FRAC_PI_6: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(111, 212));
    const FRAC_PI_8: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11, 28));
    const PI: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(22, 7));
    const SQRT_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(239, 169));
    const TAU: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(44, 7));
}

impl Consts for GenericFraction<i16> { // MaxValue = 32,767
    const E: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(25_946, 9_545));
    const FRAC_2_PI: Self = Self::TAU;
    const FRAC_PI_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355, 226));
    const FRAC_PI_4: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355, 452));
    const FRAC_PI_6: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355,678));
    const FRAC_PI_8: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355,904));
    const PI: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355, 113));
    const SQRT_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(19601, 13860));
    const TAU: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(710, 113));
}

impl Consts for GenericFraction<u16> { // MaxValue = 65,535
    const E: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(25_946, 9_545));
    const FRAC_2_PI: Self = Self::TAU;
    const FRAC_PI_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355, 226));
    const FRAC_PI_4: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355, 452));
    const FRAC_PI_6: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355,678));
    const FRAC_PI_8: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355,904));
    const PI: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(355, 113));
    const SQRT_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(19601, 13860));
    const TAU: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(710, 113));
}


impl Consts for GenericFraction<u32> {// 4_294_967_295
    const E: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(848_456_353, 312_129_649));
    const FRAC_2_PI: Self = Self::TAU;
    const FRAC_PI_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 364913));
    const FRAC_PI_4: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11_146_408, 364913));
    const FRAC_PI_6: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 1094739));
    const FRAC_PI_8: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(1393301, 364913));
    const PI: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(22292816, 364913));
    const SQRT_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(1_855_077_841, 1_311_738_121));
    const TAU: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(2292816, 364913));
}
impl Consts for GenericFraction<i32> {// 2_147_483_647
    const E: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(848_456_353, 312_129_649));
    const FRAC_2_PI: Self = Self::TAU;
    const FRAC_PI_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 364913));
    const FRAC_PI_4: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(2786602, 364913));
    const FRAC_PI_6: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 1094739));
    const FRAC_PI_8: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(1393301, 364913));
    const PI: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11_146_408, 364_913));
    const SQRT_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(1_855_077_841, 1_311_738_121));
    const TAU: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(2292816, 364913));
}

impl Consts for GenericFraction<u64> {// 18_446_744_073_709_551_615
    const E: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5_739_439_214_861_417_731_u64, 2_111_421_691_000_680_031_u64));
    const FRAC_2_PI: Self = Self::TAU;
    const FRAC_PI_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 364913));
    const FRAC_PI_4: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(2786602, 364913));
    const FRAC_PI_6: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 1094739));
    const FRAC_PI_8: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(1393301, 364913));
    const PI: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11_146_408, 364913));
    const SQRT_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(367_296_043_199, 259_717_522_849));
    const TAU: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(2292816, 364913));
}
impl Consts for GenericFraction<i64> {//  9_223_372_036_854_775_807
    const E: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5_739_439_214_861_417_731_i64, 2_111_421_691_000_680_031_i64));
    const FRAC_2_PI: Self = Self::TAU;
    const FRAC_PI_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 364913));
    const FRAC_PI_4: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(2786602, 364913));
    const FRAC_PI_6: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 1094739));
    const FRAC_PI_8: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(1393301, 364913));
    const PI: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11_146_408, 364913));
    const SQRT_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(367_296_043_199, 259_717_522_849));
    const TAU: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(2292816, 364913));
}

impl Consts for GenericFraction<usize> {//18_446_744_073_709_551_615
    const E: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5_739_439_214_861_417_731_usize, 2_111_421_691_000_680_031_usize));
    const FRAC_2_PI: Self = Self::TAU;
    const FRAC_PI_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 364913));
    const FRAC_PI_4: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(2786602, 364913));
    const FRAC_PI_6: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(5573204, 1094739));
    const FRAC_PI_8: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(1393301, 364913));
    const PI: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(11_146_408, 364913));
    const SQRT_2: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(367_296_043_199_usize, 259_717_522_849_usize));
    const TAU: Self = GenericFraction::Rational(crate::Sign::Plus, Ratio::new_raw(2292816, 364913));
}

// FRAC_PI_2