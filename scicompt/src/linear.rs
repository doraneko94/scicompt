use ndarray::*;
use ndarray_linalg::*;

use crate::float::Float;

#[derive(Clone, Copy, PartialEq)]
pub enum LinearMode {
    Regression,
    Classification,
}

pub enum Basis<T: Float> {
    Pow(usize),
    Gauss(T),
    Sigmoid(T),
}

pub struct Linear<T: Float> {
    pub mode: LinearMode,
    pub basis: Vec<fn(&Array1<T>)->T>,
}

impl<T: Float> Linear<T> {
    /*
    pub fn new(mode: LinearMode) -> Self {
        let f = |x: &Array1<T>| T::zero();
        let basis = vec![f];
        Self { mode, basis }
    }
    */
}