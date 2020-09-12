use ndarray::*;
use ndarray_linalg::*;

use crate::float::Float;
use crate::integer::Integer;

pub struct LDA<T: Float> {
    pub w: Option<Array1<T>>,
}

impl<T: Float> LDA<T> {
    pub fn new() -> Self {
        let w = None;
        Self { w }
    }

    pub fn fit<S: Integer>(&mut self, x: &Array2<T>, t: &Array1<S>) {
        let s = x.shape();
        let (n, dim) = (s[0], s[1]);
        let zero = S::zero();
        let one = S::one();
        if t.shape()[0] != n {
            panic!("aaa")
        }
        let mut m0 = Array1::<T>::zeros(dim);
        let mut m1 = Array1::<T>::zeros(dim);
        for (i, &ti) in t.iter().enumerate() {
            if ti == zero {
                m0 = m0 + x.slice(s![i, ..]);
            } else if ti == one {
                m1 = m1 + x.slice(s![i, ..]);
            } else {
                panic!("{}", ti)
            }
        }
        let mut sw = Array2::<T>::zeros((dim, dim));
        for (i, &ti) in t.iter().enumerate() {
            let xm = if ti == zero {
                (&m0 - &x.slice(s![i, ..])).into_shape((dim, 1)).unwrap()
            } else {
                (&m1 - &x.slice(s![i, ..])).into_shape((dim, 1)).unwrap()
            };
            sw = sw + xm.dot(&xm.t());
        }
        let w = sw.inv().unwrap().dot(&(m1 - m0));
        let norm = T::from(w.norm_l2()).unwrap();
        self.w = Some(w.map(|&e| e / norm));
    }

    pub fn transform(&self, x: &Array2<T>) -> Option<Array1<T>> {
        match &self.w {
            Some(w) => { Some(x.dot(w)) }
            None => { None }
        }
    }

    pub fn fit_transform<S: Integer>(&mut self, x: &Array2<T>, t: &Array1<S>) -> Option<Array1<T>> {
        self.fit(x, t);
        self.transform(x)
    }
}