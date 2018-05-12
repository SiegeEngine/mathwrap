use FullFloat;
use cgmath::{Deg, Rad};
use num_traits::{Bounded, NumCast, One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign,
               Sub, SubAssign};
use std::iter::Sum;

/// An angle.
///
/// When interoperating with numbers, functions are explicit
/// about the units of those numbers (radians, degrees or cycles).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Angle<F>(F);

impl<F: FullFloat> Angle<F> {
    /// Create an angle from radians
    #[inline]
    pub fn new_radians(radians: F) -> Angle<F> {
        Angle::<F>::from_radians(radians)
    }

    /// Create an angle from radians
    pub fn from_radians(radians: F) -> Angle<F> {
        Angle(radians)
    }

    /// Get the value of the angle as radians
    pub fn as_radians(&self) -> F {
        self.0
    }

    /// Create an angle from degrees
    #[inline]
    pub fn new_degrees(degrees: F) -> Angle<F> {
        Angle::<F>::from_degrees(degrees)
    }

    /// Create an angle from degrees
    pub fn from_degrees(degrees: F) -> Angle<F> {
        let one_eighty: F = NumCast::from(180.0_f64).unwrap();
        Angle(F::PI() * degrees / one_eighty)
    }

    /// Get the value of the angle as degrees
    pub fn as_degrees(&self) -> F {
        let one_eighty: F = NumCast::from(180.0_f64).unwrap();
        self.0 * one_eighty / F::PI()
    }

    /// Create an angle from cycles (1 cycle is a full circle)
    #[inline]
    pub fn new_cycles(cycles: F) -> Angle<F> {
        Angle::<F>::from_cycles(cycles)
    }

    /// Create an angle from cycles (1 cycle is a full circle)
    pub fn from_cycles(cycles: F) -> Angle<F> {
        let two: F = NumCast::from(2.0_f64).unwrap();
        Angle(two * F::PI() * cycles)
    }

    /// Get the value of the angle as number of cycles (full circles)
    pub fn as_cycles(&self) -> F {
        let two: F = NumCast::from(2.0_f64).unwrap();
        self.0 / (two * F::PI())
    }

    /// This normalizes to the radian range -PI to PI (half cycle each way)
    pub fn normalize_around_zero(&mut self) {
        let two: F = NumCast::from(2.0_f64).unwrap();
        self.0 = self.0 % (two * F::PI()) - F::PI();
    }

    /// This normalizes to the radian range 0 to 2*PI (full circle, positive)
    pub fn normalize_as_positive(&mut self) {
        let two: F = NumCast::from(2.0_f64).unwrap();
        self.0 = self.0 % (two * F::PI());
    }
}

impl_op!(Add, add, Angle<F>, Angle);
impl_op!(Sub, sub, Angle<F>, Angle);
impl_op!(Mul, mul, Angle<F>, Angle);
impl_op!(Div, div, Angle<F>, Angle);
impl_op!(Rem, rem, Angle<F>, Angle);

impl_aop!(AddAssign, add_assign, Angle<F>, Angle);
impl_aop!(SubAssign, sub_assign, Angle<F>, Angle);
impl_aop!(MulAssign, mul_assign, Angle<F>, Angle);
impl_aop!(DivAssign, div_assign, Angle<F>, Angle);
impl_aop!(RemAssign, rem_assign, Angle<F>, Angle);

impl_op_f!(Mul, mul, Angle<F>, Angle);
impl_op_f!(Div, div, Angle<F>, Angle);

impl_aop_f!(MulAssign, mul_assign, Angle<F>, Angle);
impl_aop_f!(DivAssign, div_assign, Angle<F>, Angle);

impl_uop!(Neg, neg, Angle<F>, Angle);

impl<F: FullFloat> Zero for Angle<F> {
    #[inline]
    fn zero() -> Angle<F> {
        Angle(F::zero())
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<F: FullFloat> One for Angle<F> {
    #[inline]
    fn one() -> Angle<F> {
        Angle(F::one())
    }
    #[inline]
    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl<F: ::approx::ApproxEq> ::approx::ApproxEq for Angle<F> {
    type Epsilon = F::Epsilon;

    #[inline]
    fn default_epsilon() -> F::Epsilon {
        F::default_epsilon()
    }

    #[inline]
    fn default_max_relative() -> F::Epsilon {
        F::default_max_relative()
    }

    #[inline]
    fn default_max_ulps() -> u32 {
        F::default_max_ulps()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: F::Epsilon, max_relative: F::Epsilon) -> bool {
        F::relative_eq(&self.0, &other.0, epsilon, max_relative)
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: F::Epsilon, max_ulps: u32) -> bool {
        F::ulps_eq(&self.0, &other.0, epsilon, max_ulps)
    }
}

impl<F: ::float_cmp::ApproxEq<Flt = F> + ::float_cmp::Ulps> ::float_cmp::ApproxEq for Angle<F> {
    type Flt = F;

    #[inline]
    fn approx_eq(
        &self,
        other: &Self,
        epsilon: Self::Flt,
        ulps: <Self::Flt as ::float_cmp::Ulps>::U,
    ) -> bool {
        self.0.approx_eq(&other.0, epsilon, ulps)
    }
}

impl<F: FullFloat> Bounded for Angle<F> {
    #[inline]
    fn min_value() -> Angle<F> {
        Angle(F::min_value())
    }
    #[inline]
    fn max_value() -> Angle<F> {
        Angle(F::max_value())
    }
}

impl<F: FullFloat> Sum<Angle<F>> for Angle<F> {
    #[inline]
    fn sum<I>(iter: I) -> Angle<F>
    where I: Iterator<Item = Angle<F>> {
        iter.fold(Angle::zero(), Add::add)
    }
}

impl<F> From<Rad<F>> for Angle<F> {
    #[inline]
    fn from(rad: Rad<F>) -> Angle<F> {
        Angle(rad.0)
    }
}

impl<F> Into<Rad<F>> for Angle<F> {
    #[inline]
    fn into(self) -> Rad<F> {
        Rad(self.0)
    }
}

impl<F> From<Deg<F>> for Angle<F>
where
    F: FullFloat,
{
    #[inline]
    fn from(deg: Deg<F>) -> Angle<F> {
        Angle::from_degrees(deg.0)
    }
}

impl<F> Into<Deg<F>> for Angle<F>
where
    F: FullFloat,
{
    #[inline]
    fn into(self) -> Deg<F> {
        Deg(self.as_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;
    use std::f32::consts::PI;
    use float_cmp::ApproxEq;

    #[test]
    fn test_radians() {
        let f: f32 = 1.234;
        let a = Angle::from_radians(f);
        assert_eq!(a.as_radians(), f);
    }

    #[test]
    fn test_degrees() {
        let f: f32 = 1.234;
        let a = Angle::from_degrees(f);
        assert_eq!(a.as_degrees(), f);
    }

    #[test]
    fn test_cycles() {
        let f: f32 = 1.234;
        let a = Angle::from_cycles(f);
        assert_eq!(a.as_cycles(), f);
    }

    #[test]
    fn test_relations() {
        let h1 = Angle::from_radians(PI);
        let h2 = Angle::from_degrees(180.0);
        let h3 = Angle::from_cycles(0.5);
        assert!(h1.approx_eq(&h2, 2.0 * EPSILON, 2));
        assert!(h1.approx_eq(&h3, 2.0 * EPSILON, 2));
        assert!(h2.approx_eq(&h3, 2.0 * EPSILON, 2));
    }

    #[test]
    fn test_assignops_autoimpl() {
        let full = Angle::from_cycles(1.0);
        let mut h1 = Angle::from_radians(PI);
        let h2 = Angle::from_degrees(180.0);
        h1 += h2;
        assert!(h1.approx_eq(&full, 2.0 * EPSILON, 2));
    }
}
