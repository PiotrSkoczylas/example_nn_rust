use libm::tanh;
use std::vec::Vec;

pub struct Neuron {
    weights: Vec<f64>,
}

impl Neuron {
    pub fn activation(&self, x: f64) -> f64 {
        tanh(x)
    }

    pub fn fire(&self, input: f64) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::new();
        for weight in &self.weights {
            result.push(weight * self.activation(input));
        }
        result
    }

    pub fn new(n: i64) -> Neuron {
        let mut tmp = Neuron {
            weights: Vec::new(),
        };
        for _i in 0..n {
            tmp.weights.push(0.0);
        }
        tmp
    }

    pub fn load_weights(&mut self, x: Vec<f64>) {
        self.weights.clear();
        for weight in x {
            self.weights.push(weight);
        }
    }
}
