use ndarray::*;
use std::collections::HashSet;

use crate::float::Float;
use crate::kernel::Kernel;

pub struct SVM<T: Float> {
    pub at: Option<Array1<T>>,
    pub b: Option<T>,
    pub support_vector: Option<Vec<Array1<T>>>,
    pub kernel: Kernel<T>,
}

impl<T: Float> SVM<T> {
    pub fn new(kernel: Kernel<T>) -> Self {
        let (at, b, support_vector) = (None, None, None);
        Self { at, b, support_vector, kernel }
    }

    pub fn y_one(&self, x: &Array1<T>) -> T {
        match (self.at.as_ref(), self.b, self.support_vector.as_ref()) {
            (Some(at), Some(b), Some(support_vector)) => {
                at.dot(&self.kernel.eval_multi(x, support_vector)) + b
            }
            _ => panic!(),
        }
    }

    pub fn y(&self, x: &Vec<Array1<T>>) -> Array1<T> {
        match (self.at.as_ref(), self.b, self.support_vector.as_ref()) {
            (Some(at), Some(b), Some(support_vector)) => {
                Array::from(
                    x.iter().map(|xk| {
                        at.dot(&self.kernel.eval_multi(&xk, support_vector)) + b
                    }).collect::<Vec<T>>()
                )
            }
            _ => panic!(),
        }
    }

    pub fn predict_one(&self, x: &Array1<T>) -> i8 {
        if self.y_one(x) >= T::zero() { 1 }
        else { -1 }
    }

    pub fn predict(&self, x: &Vec<Array1<T>>) -> Array1<i8> {
        Array::from(
            self.y(x).iter().map(|&yi| if yi >= T::zero() { 1 } else { -1 }).collect::<Vec<i8>>()
        )
    }

    pub fn fit(&mut self, x: &Vec<Array1<T>>, t: &Array1<i8>) {
        let mut smo = SMO::new(x, t, T::one(), self.kernel, T::from(1e-7).unwrap());
        smo.fit();
        let mut at_vec = Vec::new();
        let mut b = T::zero();
        let mut support_vector = Vec::new();
        smo.params(&mut at_vec, &mut b, &mut support_vector);
        self.at = Some(Array::from(at_vec));
        self.b = Some(b);
        self.support_vector = Some(support_vector);
    }
}

pub fn def_func_indexed<T: Float>(at: &Array1<T>, x: &Array1<T>, support_vector: &Vec<Array1<T>>, b: T, kernel: Kernel<T>, index: &HashSet<usize>) -> T {
    let at_indexed = Array::from(
        index.iter().map(|&i| at[i]).collect::<Vec<T>>()
    );
    at_indexed.dot(&kernel.eval_indexed(x, support_vector, index)) + b
}

pub fn obj_func_indexed<T: Float>(a: &Array1<T>, t: &Array1<T>, x: &Vec<Array1<T>>, kernel: Kernel<T>, index: &HashSet<usize>) -> T {
    let index = index.iter().map(|&i| i).collect::<Vec<usize>>();
    let a_indexed = Array::from(index.iter().map(|&i| a[i]).collect::<Vec<T>>());
    let t_indexed = Array::from(index.iter().map(|&i| t[i]).collect::<Vec<T>>());
    let at_indexed = &a_indexed * &t_indexed;
    let a_sum = a_indexed.sum();
    let l = index.len();
    a_sum - (0..l).map(|n| {
        (0..l).map(|m| {
            at_indexed[n] * at_indexed[m] * kernel.eval(&x[index[n]], &x[index[m]])
        }).sum::<T>()
    }).sum::<T>() / T::from(2).unwrap()
}

pub fn max<T: Float>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

pub fn min<T: Float>(a: T, b: T) -> T {
    if a > b { b } else { a }
}

pub struct SMO<'a, T: Float> {
    pub a: Array1<T>,
    pub a0: HashSet<usize>,
    pub a0c: HashSet<usize>,
    pub ac: HashSet<usize>,

    pub x: &'a Vec<Array1<T>>,
    pub t: Array1<T>,
    pub b: T,

    pub c: T,
    pub kernel: Kernel<T>,
    pub tolerance: T,
}

impl<'a, T: Float> SMO<'a, T> {
    pub fn new(x: &'a Vec<Array1<T>>, t: &Array1<i8>, c: T, kernel: Kernel<T>, tolerance: T) -> Self {
        let t = Array::from(t.iter().map(|&ti| T::from(ti).unwrap()).collect::<Vec<T>>());
        let n = t.shape()[0];
        let a = Array::zeros(n);
        let a0 = (0..n).collect();
        let (a0c, ac) = (HashSet::new(), HashSet::new());
        let b = T::zero();
        Self { a, a0, a0c, ac, x, t, b, c, kernel, tolerance }
    }

    pub fn update_b(&mut self) {
        //let nm = self.a0c.len();
        //if nm == 0 { return; }
        let at = &self.a * &self.t;
        let mut s = self.a0c.clone();
        s.extend(&self.ac);
        let nm = s.len();
        if nm == 0 { return; }
        self.b = s.iter().map(|&n| {
            self.t[n] - def_func_indexed(&at, &self.x[n], &self.x, self.b, self.kernel, &s)
        }).sum::<T>() / T::from(nm).unwrap();
    }

    pub fn examine(&mut self) -> bool {
        let one = T::one();
        let at = &self.a * &self.t;
        let mut index = self.a0c.clone();
        index.extend(&self.ac);
        let a0c = self.a0c.clone();
        for &i in a0c.iter() {
            let y2 = def_func_indexed(&at, &self.x[i], self.x, self.b, self.kernel, &index);
            let ty2 = self.t[i] * y2;
            if ty2 > one + self.tolerance || ty2 < one - self.tolerance {
                if self.example(i, y2, self.t[i], &at, &index) { return true; }
            }
        }
        
        let a0 = self.a0.clone();
        for &i in a0.iter() {
            let y2 = def_func_indexed(&at, &self.x[i], self.x, self.b, self.kernel, &index);
            if self.t[i] * y2 < one {
                if self.example(i, y2, self.t[i], &at, &index) { return true; };
            }
        }

        let ac = self.ac.clone();
        for &i in ac.iter() {
            let y2 = def_func_indexed(&at, &self.x[i], self.x, self.b, self.kernel, &index);
            if self.t[i] * y2 > one {
                if self.example(i, y2, self.t[i], &at, &index) { return true; };
            }
        }
        false
    }

    pub fn example(&mut self, i2: usize, y2: T, t2: T, at: &Array1<T>, index: &HashSet<usize>) -> bool {
        let e2 = y2 - t2;
        let (n0, n0c, nc) = (self.a0.len(), self.a0c.len(), self.ac.len());
        let mut i1 = 0;
        let mut e1 = if e2 > T::zero() {
            T::infinity()
        } else {
            T::neg_infinity()
        };
        for i1_new in 0..n0+n0c+nc {
            if i1_new == i2 { 
                continue;
            }
            let y1 = def_func_indexed(at, &self.x[i1_new], self.x, self.b, self.kernel, index);
            let e1_new = y1 - self.t[i1_new];
            if (e1_new < e1 && e2 > T::zero()) || (e1_new > e1 && e2 <= T::zero()) {
                e1 = e1_new;
                i1 = i1_new;
            }
        }
        if self.step(i1, i2, y2, at, index) {
            return true;
        }

        let start = 0;
        let a0c = self.a0c.iter().map(|&e| e).collect::<Vec<usize>>();
        for i in 0..n0c {
            let j = (i + start) % n0c;
            let i1 = a0c[j];
            if self.step(i1, i2, y2, at, index) {
                return true;
            }
        }

        let ncor0 = n0 + nc;
        let start = 0;
        let a0 = self.a0.iter().map(|&e| e).collect::<Vec<usize>>();
        let ac = self.ac.iter().map(|&e| e).collect::<Vec<usize>>();
        for i in 0..ncor0 {
            let j = (i +  start) % ncor0;
            let i1 = if j < n0 {
                a0[j]
            } else {
                ac[j - n0]
            };
            if self.step(i1, i2, y2, at, index) {
                return true;
            }
        }

        false
    }

    pub fn step(&mut self, i1: usize, i2: usize, y2: T, at: &Array1<T>, index: &HashSet<usize>) -> bool {
        if i1 == i2 {
            return false;
        }
        let (a1, a2) = (self.a[i1], self.a[i2]);
        let (t1, t2) = (self.t[i1], self.t[i2]);
        let s = t1 * t2;
        let y1 = def_func_indexed(at, &self.x[i1], &self.x, self.b, self.kernel, index);
        let (e1, e2) = (y1 - t1, y2 - t2);
        let (l, h) = if s < T::zero() {
            (max(T::zero(), a2 - a1), min(self.c, self.c - a2 + a1))
        } else {
            (max(T::zero(), a1 + a2 - self.c), min(self.c, a1 + a2))
        };
        if l >= h {
            return false;
        }
        let k11 = self.kernel.eval(&self.x[i1], &self.x[i1]);
        let k12 = self.kernel.eval(&self.x[i1], &self.x[i2]);
        let k22 = self.kernel.eval(&self.x[i2], &self.x[i2]);
        let eta = k11 + k22 - (k12 + k12);
        if eta > T::zero() {
            self.a[i2] = min(max(a2 + t2 * (e1 - e2) / eta, l), h);
        } else {
            let mut a = self.a.clone();
            a[i2] = l;
            let lobj = obj_func_indexed(&a, &self.t, &self.x, self.kernel, index);
            a[i2] = h;
            let hobj = obj_func_indexed(&a, &self.t, &self.x, self.kernel, index);
            if lobj < hobj - self.tolerance {
                self.a[i2] = l;
            } else if lobj > hobj + self.tolerance {
                self.a[i2] = h;
            }
        }
        if (self.a[i2] - a2).abs() < self.tolerance * (self.a[i2] + a2 + self.tolerance) {
            return false;
        }

        self.a[i1] = a1 + s * (a2 - self.a[i2]);
        if self.a[i1] < T::zero() {
            self.a[i1] = T::zero();
        } else if self.a[i1] > self.c {
            self.a[i1] = self.c;
        }

        if self.a[i1] < self.tolerance {
            if self.a0.insert(i1) {
                if a1 + self.tolerance > self.c {
                    self.ac.remove(&i1);
                } else {
                    self.a0c.remove(&i1);
                }
            }
        } else if self.a[i1] + self.tolerance <= self.c {
            if self.a0c.insert(i1) {
                if a1 + self.tolerance > self.c {
                    self.ac.remove(&i1);
                } else {
                    self.a0.remove(&i1);
                }
            }
        } else {
            if self.ac.insert(i1) {
                if a1 < self.tolerance {
                    self.a0.remove(&i1);
                } else {
                    self.a0c.remove(&i1);
                }
            }
        }

        if self.a[i2] < self.tolerance {
            if self.a0.insert(i2) {
                if a2 + self.tolerance > self.c {
                    self.ac.remove(&i2);
                } else {
                    self.a0c.remove(&i2);
                }
            }
        } else if self.a[i2] + self.tolerance <= self.c {
            if self.a0c.insert(i2) {
                if a2 + self.tolerance > self.c {
                    self.ac.remove(&i2);
                } else {
                    self.a0.remove(&i2);
                }
            }
        } else {
            if self.ac.insert(i2) {
                if a2 < self.tolerance {
                    self.a0.remove(&i2);
                } else {
                    self.a0c.remove(&i2);
                }
            }
        }
        self.update_b();
        true
    }

    pub fn fit(&mut self) {
        loop {
            if !self.examine() {
                break;
            }
        }
    }

    pub fn params(&self, at: &mut Vec<T>, b: &mut T, support_vector: &mut Vec<Array1<T>>) {
        *b = self.b;
        let n = self.a0c.len() + self.ac.len();
        *at = Vec::with_capacity(n);
        *support_vector = Vec::with_capacity(n);

        for &i in self.a0c.iter() {
            at.push(self.a[i] * self.t[i]);
            support_vector.push(self.x[i].clone());
        }
        for &i in self.ac.iter() {
            at.push(self.a[i] * self.t[i]);
            support_vector.push(self.x[i].clone());
        }
    } 
}