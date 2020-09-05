use ndarray_linalg::*;
use std::ops::DivAssign;

pub trait Float: Scalar + Lapack + PartialOrd + DivAssign {}

impl Float for f64 {}
impl Float for f32 {}