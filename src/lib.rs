#[warn(non_camel_case_types)]

extern "C" {
    fn hs_floatToHalf(f: f32) -> u16; 
    fn hs_halfToFloat(c: u16) -> f32; 
}

#[derive(Debug, Copy, Clone)]
pub struct f16 {
    bit_repr : u16,
}

impl From<u16> for f16 {
    fn from(x: u16) -> Self {
        Self {
            bit_repr: x,
        }
    }
}

impl From<f16> for u16 {
    fn from(x: f16) -> Self {
        x.bit_repr
    }
}

impl From<f16> for f32 {
    fn from(x: f16) -> Self {
        unsafe {
            hs_halfToFloat(u16::from(x))
        }
    }
}

impl From<f32> for f16 {
    fn from(x: f32) -> Self {
        unsafe {
            f16::from(hs_floatToHalf(x))
        }
    }
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
        assert_eq!(x, u16::from(f16::from(x)));
    }

    #[test]
    fn float_to_half() {
        let x0: f32 = 0.0;
        let x1: f32 = 1.0;
        let x2: f32 = 2.0;
        assert_eq!(0, u16::from(f16::from(x0)));
        assert_eq!(15360, u16::from(f16::from(x1)));
        assert_eq!(16384, u16::from(f16::from(x2)));
    }
}
