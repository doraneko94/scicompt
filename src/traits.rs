use ndarray_linalg::*;
use std::ops::DivAssign;

macro_rules! impl_float {
    ($type:ty) => {
        impl Float for $type {
            const GAMMA_DK: &'static[Self] = &[
                2.48574089138753565546e-5,
                1.05142378581721974210,
                -3.45687097222016235469,
                4.51227709466894823700,
                -2.98285225323576655721,
                1.05639711577126713077,
                -1.95428773191645869583e-1,
                1.70970543404441224307e-2,
                -5.71926117404305781283e-4,
                4.63399473359905636708e-6,
                -2.71994908488607703910e-9
            ];
        }
    };
}

pub trait Float: Scalar + Lapack + PartialOrd + DivAssign {
    const GAMMA_DK: &'static[Self];
}

impl_float!(f64);
impl_float!(f32);