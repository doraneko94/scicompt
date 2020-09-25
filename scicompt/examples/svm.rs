use ndarray::*;
use scicompt::svm::*;
use scicompt::kernel::Kernel::Linear;

fn main() {
    let mut s = SVM::new(Linear);
    let x = vec![arr1(&[0.0, 0.0]), arr1(&[1.0, 1.0]), arr1(&[0.1, 0.1]), arr1(&[1.1, 1.1])];
    let t = arr1(&[-1, 1, -1, 1]);
    s.fit(&x, &t);
    println!("{:?}", s.predict(&x));
}