use ndarray::*;
use scicompt::lda::LDA;

fn main() {
    let mut lda = LDA::new();
    let x = arr2(&[
        [1.0, 3.0],
        [2.0, 3.1],
        [0.5, 2.9],
        [2.0, 1.0],
        [4.0, 2.0],
        [6.0, 3.0],
    ]);
    let t = arr1(&[0,0,0,1,1,1]);
    let y = lda.fit_transform(&x, &t);
    println!("{:?}", y);
}