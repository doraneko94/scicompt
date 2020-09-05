mod consts;
mod io;

use consts::*;
use io::save_to_file;

fn main() {
    let mut c = Consts::new();
    c.add_val("TWO_PI_SQRT", (std::f64::consts::PI * 2.0).sqrt());

    let s_trait_core = format!("\tconst {}: Self;", c.name[0]);
    let s_trait = format!("pub trait FloatConst: Float {{\n{}\n}}", &s_trait_core);
    let s_impl_core = format!("\t\t\tconst {}: Self = {};", c.name[0], c.value[0]);
    let s_impl = format!("macro_rules! impl_float_const {{\n\t($type:ty) => {{\n\t\timpl FloatConst for $type {{\n{}\n\t\t}}\n\t}};\n}}", s_impl_core);
    let s = format!("use crate::float::Float;\n\n{}\n\n{}\n\nimpl_float_const!(f64);\nimpl_float_const!(f32);", s_trait, s_impl);
    let _ = save_to_file("../scicompt/src/consts.rs", &s);
}