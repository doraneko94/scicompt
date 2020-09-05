use crate::io::save_to_file;

pub struct Consts {
    pub name: Vec<String>,
    pub value: Vec<String>,
    pub vec_name: Vec<String>,
    pub vec_value: Vec<String>,
}

impl Consts {
    pub fn new() -> Self {
        let name = Vec::new();
        let value = Vec::new();
        let vec_name = Vec::new();
        let vec_value = Vec::new();
        Self { name, value, vec_name, vec_value }
    }

    pub fn add_val(&mut self, nam: &str, val: f64) {
        self.name.push(nam.to_owned());
        self.value.push(format!("{}", val));
    }

    pub fn add_vec(&mut self, nam: &str, val: &Vec<f64>) {
        self.vec_name.push(nam.to_owned());
        self.vec_value.push(format!("{:?}", val));
    }

    pub fn save(&self) {
        let mut s_trait_core = String::new();
        let mut s_impl_core = String::new();
        for (nam, val) in self.name.iter().zip(self.value.iter()) {
            s_trait_core = s_trait_core + &format!("\n\tconst {}: Self;", nam);
            s_impl_core = s_impl_core + &format!("\n\t\t\tconst {}: Self = {};", nam, val);
        }
        for (nam, val) in self.vec_name.iter().zip(self.vec_value.iter()) {
            s_trait_core = s_trait_core + &format!("\n\tconst {}: &'static [Self];", nam);
            s_impl_core = s_impl_core + &format!("\n\t\t\tconst {}: &'static[Self] = &{};", nam, val);
        }
        let s_trait = format!("pub trait FloatConst: Float {{{}\n}}", &s_trait_core);
        let s_impl = format!("macro_rules! impl_float_const {{\n\t($type:ty) => {{\n\t\timpl FloatConst for $type {{{}\n\t\t}}\n\t}};\n}}", s_impl_core);
        let s = format!("use crate::float::Float;\n\n{}\n\n{}\n\nimpl_float_const!(f64);\nimpl_float_const!(f32);", s_trait, s_impl);
        let _ = save_to_file("../scicompt/src/consts.rs", &s);
    }
}