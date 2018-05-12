extern crate num_traits;

mod macros;

mod angle;
pub use self::angle::Angle;

use num_traits::{Float, FloatConst, NumAssignOps};
use std::fmt::Debug;

// This trait allows us to write code generic across both floating point types
pub trait FullFloat:
    Float + FloatConst + NumAssignOps + Clone + Debug + approx::ApproxEq + float_cmp::ApproxEq
{
}

impl<T> FullFloat for T
where
    T: Float + FloatConst + NumAssignOps + Clone + Debug + approx::ApproxEq + float_cmp::ApproxEq,
{
}

pub mod cgmath {
    extern crate cgmath;
    pub use self::cgmath::{Deg, Rad};
}

pub mod nalgebra {
    extern crate nalgebra;
}

pub mod approx {
    extern crate approx;
    pub use self::approx::ApproxEq;
}

pub mod float_cmp {
    extern crate float_cmp;
    pub use self::float_cmp::{ApproxEq, Ulps};
}
