use ndarray::*;
use ndarray_linalg::*;
use num_traits::{Float, FromPrimitive};

pub trait F: Float + FromPrimitive + std::iter::Sum + Scalar + Lapack {}
impl F for f64{}
impl F for f32{}

pub struct PCA<T: F> {
    /// n_components x n_features
    pub components: Option<Array2<T>>,
    pub eigenvals: Option<Array1<<T as Scalar>::Real>>,
}

impl<T: F> PCA<T> {
    pub fn new() -> Self {
        let components = None;
        let eigenvals = None;
        Self { components, eigenvals }
    }

    pub fn fit(&mut self, x: &Array2<T>) {
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
                let mut components = Array2::zeros((col, col));
                let mut eigenvals = Array1::zeros(col);
                for i in 0..col {
                    Zip::from(&mut components.slice_mut(s![i, ..]))
                        .and(vec.slice(s![.., col-i-1]))
                        .apply(|a, &b| {
                            *a = b;
                        });
                    eigenvals[i] = val[col-i-1];
                }
                self.components = Some(components);
                self.eigenvals = Some(eigenvals);
            }
            Err(_) => { println!("Couldn't calculate eigenvalues!") }
        }
    }

    pub fn transform(&self, x: &Array2<T>, n_components: usize) -> Option<Array2<T>> {
        match &self.components {
            Some(vec) => {
                let components = vec.slice(s![..n_components, ..]).to_owned();
                let x_mean = x.mean_axis(Axis(0)).unwrap();
                Some(components.dot(&(x - &x_mean).t()))
            }
            None => None,
        }
    }

    /// panics if self.eigenval.shape[0] <= component
    pub fn proportion_of_variance(&self, component: usize) -> Option<<T as Scalar>::Real> {
        match &self.eigenvals {
            Some(a) => {
                // let n_components = a.shape()[0];
                let lambda_sum = a.sum_axis(Axis(0)).into_scalar();
                Some(a[component] / lambda_sum)
            }
            None => None,
        }
    }

    /// panics if self.eigenval.shape[0] <= component
    pub fn cumulative_proportion_of_variance(&self, component: usize) -> Option<<T as Scalar>::Real> {
        match &self.eigenvals {
            Some(a) => {
                // let n_components = a.shape()[0];
                let lambda_sum = a.sum_axis(Axis(0)).into_scalar();
                Some(a.slice(s![..component+1]).sum_axis(Axis(0)).into_scalar() / lambda_sum)
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
}