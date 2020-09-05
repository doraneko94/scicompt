use ndarray::*;
use ndarray_linalg::*;

use crate::float::Float;

pub struct PCA<T: Float> {
    /// n_components x n_features
    pub eigvecs: Option<Array2<T>>,
    pub eigvals: Option<Array1<T>>,
    pub components: Option<Array2<T>>,
    //pub w: Option<Array2<T>>,
    //pub alpha: Option<Array1<T>>,
    //pub var: Option<Array1<T>>,
}

impl<T: Float> PCA<T> {
    pub fn new() -> Self {
        let eigvecs = None;
        let eigvals = None;
        let components = None;
        Self { eigvecs, eigvals, components }
    }

    pub fn fit(&mut self, x: &Array2<T>, n_components: usize) {
        let shape = x.shape();
        let row = shape[0];
        let col = shape[1];
        let e = x.mean_axis(Axis(0)).unwrap();
        let mut s: Array2<T> = Array2::zeros((col, col));
        let n_1 = T::from_usize(row - 1).unwrap();

        for r in 0..col {
            for c in 0..col {
                s[[r, c]] = x.slice(s![.., r]).iter().zip(x.slice(s![.., c]).iter()).map(|(&er, &ec)| (er - e[r]) * (ec - e[c])).sum::<T>() / n_1;
            }
        }
        match s.eigh(UPLO::Upper) {
            Ok((val, vec)) => {
                self.eigvecs = Some(vec);
                self.eigvals = Some(val.map(|&e| T::from(e).unwrap()));
                self.to_components(n_components);
            }
            Err(_) => { panic!("Couldn't calculate eigenvalues!") }
        }
    }

    pub fn to_components(&mut self, n_components: usize) {
        match &self.eigvecs {
            Some(vec) => {
                let mut components = Array2::zeros((n_components, n_components));
                for i in 0..n_components {
                    Zip::from(&mut components.slice_mut(s![i, ..]))
                        .and(vec.slice(s![.., n_components-i-1]))
                        .apply(|a, &b| {
                            *a = b;
                        });
                }
                self.components = Some(components);
            }
            None => { panic!("PCA has not been fitted!"); }
        }
    }

    pub fn transform(&self, x: &Array2<T>) -> Option<Array2<T>> {
        match &self.components {
            Some(vec) => {
                let x_mean = x.mean_axis(Axis(0)).unwrap();
                Some(vec.dot(&(x - &x_mean).t()))
            }
            None => None,
        }
    }

    /// panics if self.eigenval.shape[0] <= component
    pub fn proportion_of_variance(&self, component: usize) -> Option<T> {
        match &self.eigvals {
            Some(val) => {
                let n_components = val.shape()[0];
                let lambda_sum = val.sum_axis(Axis(0)).into_scalar();
                Some(val[n_components - component - 1] / lambda_sum)
            }
            None => None,
        }
    }

    /// panics if self.eigenval.shape[0] <= component
    pub fn cumulative_proportion_of_variance(&self, component: usize) -> Option<T> {
        match &self.eigvals {
            Some(val) => {
                let n_components = val.shape()[0];
                let lambda_sum = val.sum_axis(Axis(0)).into_scalar();
                Some(val.slice(s![n_components - component - 1..]).sum_axis(Axis(0)).into_scalar() / lambda_sum)
            }
            None => None,
        }
    }

    pub fn components_vec(&self) -> Option<Vec<Array1<T>>> {
        match &self.components {
            Some(vec) => {
                let n_components = vec.shape()[0];
                Some((0..n_components).map(|i| vec.slice(s![i, ..]).to_owned()).collect())
            }
            None => None,
        }
    }

    /*
    pub fn fit_bayesian(&mut self, x: &Array2<T>, n_components: usize) {
        let ndim = x.shape()[1];
        self.fit(x, n_components);
        let vec = self.eigvecs.as_ref().unwrap();
        let val = self.eigvals.as_ref().unwrap();
        
        let (index, mut var) = if ndim <= n_components {
            (0, T::zero())
        } else {
            let i = ndim - n_components;
            let v = val.slice(s![..i]).mean_axis(Axis(0)).unwrap().into_scalar();
            (i, T::from(v).unwrap())
        };

        let mut w = vec.slice(s![.., index..]).dot(&Array::from_diag(&val.slice(s![index..]).map(|e| e.sqrt())))
                    - &Array2::from_diag(&arr1(&vec![var; n_components]));
        let mut alpha = arr1(&vec![T::from(ndim).unwrap(); n_components]);
        Zip::from(&mut alpha).and(&(&w * &w).sum_axis(Axis(0))).apply(|a, &b| {
            *a /= b;
        });

        let d = x - &x.mean_axis(Axis(0)).unwrap();

        for _ in 0..100 {

        }
    }

    fn expectation(&self, d: &Array2<T>) {}
    */
}