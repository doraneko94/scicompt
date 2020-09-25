use ndarray::*;
use std::collections::HashSet;

use crate::float::Float;

#[derive(Clone, Copy)]
pub enum Kernel<T: Float> {
    Linear,
    Poly(T, T, i32),
    RBF(T),
    Sigmoid(T, T),
}

impl<T: Float> Kernel<T> {
    pub fn eval(&self, x1: &Array1<T>, x2: &Array1<T>) -> T {
        match self {
            Kernel::Linear => x1.dot(x2),
            Kernel::Poly(gamma, r, d) => (*gamma * x1.dot(x2) + *r).powi(*d),
            Kernel::RBF(gamma) => {
                let dx = x1 - x2;
                (-(*gamma) * dx.dot(&dx)).exp()
            }
            Kernel::Sigmoid(gamma, r) => (*gamma * x1.dot(x2) + *r).tanh(),
        }
    }

    pub fn eval_multi(&self, x: &Array1<T>, xn: &Vec<Array1<T>>) -> Array1<T> {
        let n = xn.len();
        Array::from(
            (0..n).map(|i| {
                self.eval(x, &xn[i])
            }).collect::<Array1<T>>()
        )
    }

    pub fn eval_indexed(&self, x: &Array1<T>, xn: &Vec<Array1<T>>, index: &HashSet<usize>) -> Array1<T> {
        Array::from(
            index.iter().map(|&i| {
                self.eval(x, &xn[i])
            }).collect::<Array1<T>>()
        )
    }
}