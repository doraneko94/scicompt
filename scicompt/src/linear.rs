use ndarray::*;
use ndarray_linalg::*;

use crate::float::Float;

#[derive(Clone, Copy, PartialEq)]
pub enum LinearMode {
    Regression,
    Classification,
}

#[derive(Clone, PartialEq)]
pub enum Basis<T: Float> {
    Poly(T),
    Gauss((T, T)),
    Sigmoid((T, T)),
}

impl<T: Float> Basis<T> {
    pub fn eval(&self, x: &Array1<T>) -> Array1<T> {
        match self {
            Basis::Poly(i) => {
                x.map(|e| e.pow(*i))
            }
            Basis::Gauss((mu, s)) => {
                let xm = x - mu;
                let two = T::one() + T::one();
                xm.map(|&e| (-e*e/(two*(*s))).exp())
            }
            Basis::Sigmoid((mu, s)) => {
                let xm = x - mu;
                let one = T::one();
                xm.map(|&e| one / (one + (-e/(*s)).exp()))
            }
        }
    }
}

pub struct Linear<T: Float> {
    pub mode: LinearMode,
    pub basis: Vec<Basis<T>>,
}

impl<T: Float> Linear<T> {
    pub fn new(mode: LinearMode, b: &Vec<Basis<T>>) -> Self {
        let basis = b.to_vec();
        Self { mode, basis }
    }

    pub fn design_matrix(&self, x: &Array2<T>) -> Array2<T> {
        let s = x.shape();
        let (n, dim) = (s[0], s[1]);
        Array2::from(
            (0..n).map(
                |i| {
                    let xi = x.slice(s![i, ..]).to_owned();
                    self.basis.iter()
                              .map(|phi| phi.eval(&xi))
                              .collect::<Vec<T>>()
                }
            )
        )
    }
}