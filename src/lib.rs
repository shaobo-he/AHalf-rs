#[warn(non_camel_case_types)]

use std::cmp::Ordering;

extern "C" {
    fn hs_floatToHalf(f: f32) -> u16; 
    fn hs_halfToFloat(c: u16) -> f32; 
}

// the type `f16`, which is a wrapper around a u16
#[derive(Debug, Copy, Clone)]
pub struct f16 {
    bit_repr : u16,
}

impl f16 {
    // bit conversions
    fn from_bits(x: u16) -> Self {
        Self {
            bit_repr: x,
        }
    }

    fn to_bits(self) -> u16 {
        self.bit_repr
    }
}

// conversions from/to f32
impl From<f16> for f32 {
    fn from(x: f16) -> Self {
        unsafe {
            hs_halfToFloat(f16::to_bits(x))
        }
    }
}

impl From<f32> for f16 {
    fn from(x: f32) -> Self {
        unsafe {
            f16::from_bits(hs_floatToHalf(x))
        }
    }
}

macro_rules! bin_pred {
    ($pred_name:ident, $ret_type:ty) => {
        fn $pred_name(&self, other: &Self) -> $ret_type {
            let lhs = f32::from(*self);
            let rhs = &f32::from(*other);
            lhs.$pred_name(rhs)
        }
    }
}

// partial equality
impl PartialEq for f16 {
    bin_pred!(eq, bool);
    bin_pred!(ne, bool);
}

// partial order
impl PartialOrd for f16 {
    bin_pred!(partial_cmp, Option<Ordering>);
}

#[cfg(test)]
mod tests {
    use crate::f16;
    #[test]
    fn size_check() {
        use std::mem;
        assert_eq!(mem::size_of::<f16>(), 2);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn identity_check() {
        let x = 1;
        let xs = f16 {
            bit_repr: x,
        };
        assert_eq!(x, xs.bit_repr);
    }

    #[test]
    fn bits_and_half() {
        let x = 1;
        assert_eq!(x, f16::from_bits(x).to_bits());
    }

    #[test]
    fn float_to_half() {
        let x0: f32 = 0.0;
        let x1: f32 = 1.0;
        let x2: f32 = 2.0;
        assert_eq!(0, f16::to_bits(f16::from(x0)));
        assert_eq!(15360, f16::from(x1).to_bits());
        assert_eq!(16384, f16::from(x2).to_bits());
    }

    #[test]
    fn half_to_float() {
        let x0: f16 = f16::from_bits(0);
        let x1: f16 = f16::from_bits(15360);
        let xmin: f16 = f16::from_bits(1);
        assert_eq!(0.0, f32::from(x0));
        assert_eq!(1.0, f32::from(x1));
        assert_eq!(5.9604645e-8, f32::from(xmin));
    }

    #[test]
    fn partial_eq_check() {
        let x0: f16 = f16::from_bits(0);
        let x1: f16 = f16::from_bits(1);
        assert!(x0 != x1);
    }

    #[test]
    fn partial_ord_check() {
        let x0: f16 = f16::from_bits(0);
        let x1: f16 = f16::from_bits(1);
        assert!(x0 < x1);
    }
}
