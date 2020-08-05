use ndarray::*;
use scicompt::*;

fn main() {
    let mut pca = PCA::new();
    let data = arr2(&[
        [7.0, 4.0, 3.0],
        [4.0, 1.0, 8.0],
        [6.0, 3.0, 5.0],
        [8.0, 6.0, 1.0],
        [8.0, 5.0, 7.0],
        [7.0, 2.0, 9.0],
        [5.0, 3.0, 3.0],
        [9.0, 5.0, 8.0],
        [7.0, 4.0, 5.0],
        [8.0, 2.0, 2.0]
    ]);
    pca.fit(&data);
    println!("{:?}", pca.components);
    println!("{:?}", pca.proportion_of_variance(0));
    println!("{:?}", pca.proportion_of_variance(1));
    println!("{:?}", pca.cumulative_proportion_of_variance(1));
    println!("{:?}", pca.components_vec());
    println!("{:?}", pca.transform(&data, 2));
}