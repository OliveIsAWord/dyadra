use std::fmt;
use std::ops::{Mul};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dyadra(u64);

impl Dyadra {
    pub const MIN: Self = Self(1 << 63);
    pub const MAX: Self = Self(0);
    fn leading_one(self) -> u64 {
        (u64::BITS - self.0.leading_zeros()).into()
    }
    fn mantissa(self) -> u64 {
        let i = self.leading_one();
        let mantissa_mask = (1 << i.saturating_sub(1)) - 1;
        let mantissa_bits = self.0 & mantissa_mask;
        (mantissa_bits << 1) + 1
    }
    // TODO: make an `overflowing_mul -> (bool, Self)` which can do lossy multiply
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let i1 = self.leading_one();
        let i2 = rhs.leading_one();
        let i_product = i1 + i2;
        if i_product >= u64::BITS.into() { return None }
        let high_bit = i_product.checked_sub(1).map_or(0, |x| 1 << x);
        let mantissa_product = self.mantissa() * rhs.mantissa();
        let mantissa_bits = mantissa_product >> 1;
        // TODO: This condition always holds... right?
        debug_assert!(mantissa_bits < high_bit || mantissa_bits == 0 && high_bit == 0);
        Some(Self(high_bit + mantissa_bits))
    }
    #[must_use]
    pub fn to_float(self) -> f64 {
        let i = self.leading_one();
        let mantissa = self.mantissa();
        #[allow(clippy::cast_precision_loss)]
        {
            // REASON: `i` will always be <= 64, so can be converted losslessly. The caller accepts
            // the potential precision loss from converting `mantissa`.
            mantissa as f64 / (i as f64).exp2()
        }
    }
    // TODO: other ops
}

impl Mul<Self> for Dyadra {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.checked_mul(rhs).expect("Precision error")
    }
}

impl fmt::Debug for Dyadra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dyadra({}/2^{})", self.mantissa(), self.leading_one())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let d = Dyadra(0b101);
        assert_eq!(d.to_float(), 0.375);
    }
    #[test]
    fn edge_cases() {
        assert_eq!(Dyadra(0).to_float(), 1.0);
        assert_eq!(Dyadra(1).to_float(), 0.5);
        assert_eq!(Dyadra(2).to_float(), 0.25);
        assert_eq!(Dyadra(3).to_float(), 0.75);
        assert_eq!(Dyadra(4).to_float(), 0.125);
    }

    #[test]
    fn mult() {
        assert_eq!(Dyadra(0) * Dyadra(0), Dyadra(0));
        assert_eq!(Dyadra(0b101) * Dyadra(0b1011), Dyadra(0b1001010));
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Dyadra(0b101)), "Dyadra(3/2^3)");
        assert_eq!(format!("{:?}", Dyadra::MIN), "Dyadra(1/2^64)");
        assert_eq!(format!("{:?}", Dyadra::MAX), "Dyadra(1/2^0)");
    }
}
