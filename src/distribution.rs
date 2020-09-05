use crate::float::Float;

pub trait Distribution<T: Float> {
    fn mean(&self) -> T;
    fn var(&self) -> T;
    fn std(&self) -> T {
        self.var().sqrt()
    }
    fn mode(&self) -> T;
}

pub struct Bernoulli<T: Float> {
    pub mu: T,
}

impl<T: Float> Distribution<T> for Bernoulli<T> {
    fn mean(&self) -> T {
        self.mu
    }

    fn var(&self) -> T {
        self.mu * (T::one() - self.mu)
    }

    fn mode(&self) -> T {
        if self.mu >= T::from(0.5).unwrap() {
            T::one()
        } else {
            T::zero()
        }
    }
}

pub struct Beta<T: Float> {
    pub a: T,
    pub b: T,
}

impl<T: Float> Distribution<T> for Beta<T> {
    fn mean(&self) -> T {
        let a = self.a;
        let b = self.b;
        a / (a + b)
    }

    fn var(&self) -> T {
        let a = self.a;
        let b = self.b;
        let a_plus_b = a + b;
        a * b / (a_plus_b * a_plus_b * (a_plus_b + T::one()))
    }

    fn mode(&self) -> T {
        let a = self.a;
        let b = self.b;
        (a - T::one()) / (a + b - T::from(2).unwrap()) 
    }
}

pub struct Gaussian<T: Float> {
    pub mu: T,
    pub sigma2: T,
}

impl<T: Float> Distribution<T> for Gaussian<T> {
    fn mean(&self) -> T {
        self.mu
    }

    fn var(&self) -> T {
        self.sigma2
    }

    fn mode(&self) -> T {
        self.mu
    }
}