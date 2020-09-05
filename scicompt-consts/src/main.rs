mod consts;
mod io;

use consts::*;

fn main() {
    let mut c = Consts::new();
    c.add_val("TWO_PI_SQRT", (std::f64::consts::PI * 2.0).sqrt());
    c.add_val("TWO_PI_SQRT_INV", 1.0 / (std::f64::consts::PI * 2.0).sqrt());
    c.add_vec("DUMMY", &vec![1.0, 2.0, 3.0]);
    c.save();
}