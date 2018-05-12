extern crate cgmath;
extern crate nalgebra;
extern crate num_traits;

use num_traits::{Float, FloatConst, NumAssignOps};
use std::fmt::Debug;

// This trait allows us to write code generic across both floating point types
pub trait FullFloat: Float + FloatConst + NumAssignOps + Clone + Debug {}

impl<T> FullFloat for T
where
    T: Float + FloatConst + NumAssignOps + Clone + Debug,
{
}
