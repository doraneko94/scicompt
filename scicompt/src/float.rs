use ndarray_linalg::*;
use num_traits::float;
use std::ops::DivAssign;

pub trait Float: float::Float + Scalar + Lapack + PartialOrd + DivAssign {}

impl Float for f64 {}
impl Float for f32 {}