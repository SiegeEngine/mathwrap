use FullFloat;
use approx::ApproxEq as AApproxEq;
use cgmath::{Deg, Rad};
use float_cmp::{ApproxEq as FApproxEq, Ulps};
use num_traits::{NumCast, One, Zero};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

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

impl<F: FullFloat> Add<Angle<F>> for Angle<F> {
    type Output = Angle<F>;

    fn add(self, rhs: Angle<F>) -> Angle<F> {
        Angle(self.0 + rhs.0)
    }
}

impl<F: FullFloat> Sub<Angle<F>> for Angle<F> {
    type Output = Angle<F>;

    fn sub(self, rhs: Angle<F>) -> Angle<F> {
        Angle(self.0 - rhs.0)
    }
}

impl<F: FullFloat> Mul<Angle<F>> for Angle<F> {
    type Output = Angle<F>;

    fn mul(self, rhs: Angle<F>) -> Angle<F> {
        Angle(self.0 * rhs.0)
    }
}

impl<F: FullFloat> Div<Angle<F>> for Angle<F> {
    type Output = Angle<F>;

    fn div(self, rhs: Angle<F>) -> Angle<F> {
        Angle(self.0 / rhs.0)
    }
}

impl<F: FullFloat> Rem<Angle<F>> for Angle<F> {
    type Output = Angle<F>;

    fn rem(self, rhs: Angle<F>) -> Angle<F> {
        Angle(self.0 % rhs.0)
    }
}

impl<F: FullFloat> Mul<F> for Angle<F> {
    type Output = Angle<F>;

    fn mul(self, rhs: F) -> Angle<F> {
        Angle(self.0 * rhs)
    }
}

impl<F: FullFloat> Div<F> for Angle<F> {
    type Output = Angle<F>;

    fn div(self, rhs: F) -> Angle<F> {
        Angle(self.0 / rhs)
    }
}

impl<F: FullFloat> Neg for Angle<F> {
    type Output = Angle<F>;

    fn neg(self) -> Angle<F> {
        Angle(-self.0)
    }
}

impl<F: FullFloat> Zero for Angle<F> {
    fn zero() -> Angle<F> {
        Angle(F::zero())
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<F: FullFloat> One for Angle<F> {
    fn one() -> Angle<F> {
        Angle(F::one())
    }
    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl<F: AApproxEq> AApproxEq for Angle<F> {
    type Epsilon = F::Epsilon;

    fn default_epsilon() -> F::Epsilon {
        F::default_epsilon()
    }

    fn default_max_relative() -> F::Epsilon {
        F::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        F::default_max_ulps()
    }

    fn relative_eq(&self, other: &Self, epsilon: F::Epsilon, max_relative: F::Epsilon) -> bool {
        F::relative_eq(&self.0, &other.0, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: F::Epsilon, max_ulps: u32) -> bool {
        F::ulps_eq(&self.0, &other.0, epsilon, max_ulps)
    }
}

impl<F: FApproxEq<Flt = F> + Ulps> FApproxEq for Angle<F> {
    type Flt = F;

    fn approx_eq(&self, other: &Self, epsilon: Self::Flt, ulps: <Self::Flt as Ulps>::U) -> bool {
        self.0.approx_eq(&other.0, epsilon, ulps)
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
        use std::f32::EPSILON;
        use std::f32::consts::PI;
        use ::float_cmp::ApproxEq;

        let h1 = Angle::from_radians(PI);
        let h2 = Angle::from_degrees(180.0);
        let h3 = Angle::from_cycles(0.5);
        assert!(h1.approx_eq(&h2, 2.0 * EPSILON, 2));
        assert!(h1.approx_eq(&h3, 2.0 * EPSILON, 2));
        assert!(h2.approx_eq(&h3, 2.0 * EPSILON, 2));
    }
}
