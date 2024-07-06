extern crate ndarray;
extern crate ndarray_rand;
extern crate smartcore;

use ndarray::{Array2, Array1, Axis};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::ensemble::random_forest_classifier::RandomForestClassifier;
use smartcore::ensemble::random_forest_classifier::RandomForestClassifierParameters;
use smartcore::metrics::accuracy;

/// Generates random data for classification.
fn generate_data(n: usize) -> (Array2<f64>, Array1<f64>) {
    let data = Array2::<f64>::random((n, 2), Uniform::new(-10.0, 10.0));
    println!("{data}");
    let labels = data.map_axis(Axis(1), |x| if x.sum() > 0.0 { 1.0 } else { 0.0 });
    (data, labels)
}

pub fn main() {
    
    
    let (features, labels) = generate_data(100);

    // Convert ndarray to DenseMatrix
    let features_matrix = DenseMatrix::from_array(features.dim().0, features.dim().1, &features.as_slice().unwrap());
    let labels_vec = labels.to_vec();

    // Define Random Forest parameters
    let rf_params = RandomForestClassifierParameters::default();

    // Train Random Forest model
    let model = RandomForestClassifier::fit(&features_matrix, &labels_vec, rf_params).unwrap();

    // Predict using the trained model
    let predictions = model.predict(&features_matrix).unwrap();

    // Calculate accuracy
    let accuracy_score = accuracy(&labels_vec, &predictions);
    println!("Accuracy: {}", accuracy_score);
}
