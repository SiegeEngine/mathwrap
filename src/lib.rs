extern crate approx;
extern crate cgmath;
extern crate float_cmp;
extern crate nalgebra;
extern crate num_traits;

mod angle;
pub use self::angle::Angle;

use approx::ApproxEq as AApproxEq;
use float_cmp::ApproxEq as FApproxEq;
use num_traits::{Float, FloatConst, NumAssignOps};
use std::fmt::Debug;

// This trait allows us to write code generic across both floating point types
pub trait FullFloat:
    Float + FloatConst + NumAssignOps + Clone + Debug + AApproxEq + FApproxEq
{
}

impl<T> FullFloat for T
where
    T: Float + FloatConst + NumAssignOps + Clone + Debug + AApproxEq + FApproxEq,
{
}
