use crate::float::Float;

pub trait FloatConst: Float {
	const TWO_PI_SQRT: Self;
}

macro_rules! impl_float_const {
	($type:ty) => {
		impl FloatConst for $type {
			const TWO_PI_SQRT: Self = 2.5066282746310002;
		}
	};
}

impl_float_const!(f64);
impl_float_const!(f32);