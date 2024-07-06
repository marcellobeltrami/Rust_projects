extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;
mod rf;


use ndarray::{Array1, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

/// Generates random _data for logistic regression.
fn generate_data(n: usize) -> (Array2<f64>, Array1<f64>) {
    let mut _rng = rand::thread_rng();
    let mut _data = Array2::<f64>::random((n, 2), Uniform::new(-10.0, 10.0));
    let mut labels = Array1::<f64>::zeros(n);

    for i in 0..n {
        let x1 = _data[[i, 0]];
        let x2 = _data[[i, 1]];
        labels[i] = if x1 + x2 > 0.0 { 1.0 } else { 0.0 };
    }

    (_data, labels)
}

/// Computes the sigmoid function.
fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

/// Predicts the probability for given features and weights.
fn predict(features: &Array2<f64>, weights: &Array1<f64>) -> Array1<f64> {
    let mut probs = Array1::zeros(features.nrows());
    for i in 0..features.nrows() {
        let z = features.row(i).dot(weights);
        probs[i] = sigmoid(z);
    }
    probs
}

/// Computes the binary cross-entropy loss.
fn compute_loss(predictions: &Array1<f64>, labels: &Array1<f64>) -> f64 {
    let mut loss = 0.0;
    for (pred, label) in predictions.iter().zip(labels.iter()) {
        loss += -label * pred.ln() - (1.0 - label) * (1.0 - pred).ln();
    }
    loss / labels.len() as f64
}

/// Performs gradient descent to optimize the weights.
fn gradient_descent(
    features: &Array2<f64>,
    labels: &Array1<f64>,
    weights: &mut Array1<f64>,
    learning_rate: f64,
    epochs: usize,
) {
    let m = labels.len() as f64;

    for _ in 0..epochs {
        let predictions = predict(features, weights);
        let mut gradient = Array1::<f64>::zeros(weights.len());

        for i in 0..features.nrows() {
            let error = predictions[i] - labels[i];
            for j in 0..features.ncols() {
                gradient[j] += error * features[[i, j]];
            }
        }

        for j in 0..weights.len() {
            weights[j] -= learning_rate * gradient[j] / m;
        }

        let loss = compute_loss(&predictions, labels);
        println!("Loss: {}", loss);
    }
}

fn main() {
    let (features, labels) = generate_data(100);

    let mut weights = Array1::<f64>::zeros(features.ncols());
    let learning_rate = 0.01;
    let epochs = 1000;

    gradient_descent(&features, &labels, &mut weights, learning_rate, epochs);

    println!("Optimized weights: {:?}", weights); 
    rf::main();
    
}
