use crate::modules::{iris, neuron};
use libm::pow;

pub struct Network {
    layers: Vec<Vec<neuron::Neuron>>,
}

impl Network {
    pub fn new() -> Self {
        Network { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, n: i64) {
        let mut tmp: Vec<neuron::Neuron> = Vec::new();
        for _i in 0..n {
            tmp.push(neuron::Neuron::new(n));
        }
        self.layers.push(tmp);
    }

    pub fn load_weights(&mut self, weights: Vec<f64>) {
        let mut i = 0;
        for j in 0..self.layers.len() - 1 {
            for k in 0..self.layers[j].len() {
                let mut tmp: Vec<f64> = Vec::new();
                for _l in 0..self.layers[j + 1].len() {
                    tmp.push(weights[i]);
                    i = i + 1;
                }
                self.layers[j][k].load_weights(tmp);
            }
        }
    }

    pub fn count_synapses(&self) -> i64 {
        let mut count: i64 = 0;
        for i in 1..self.layers.len() {
            count = count + ((self.layers[i].len() as i64) * (self.layers[i - 1].len() as i64));
        }
        count
    }

    pub fn feed_forward(&self, sample: Vec<f64>) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::new();
        let mut in_layer: Vec<f64> = Vec::new();
        for i in 0..self.layers[0].len() {
            in_layer.push(sample[i]);
        }
        for i in 0..self.layers.len() - 1 {
            let mut out_layer: Vec<f64> = Vec::new();
            for _j in 0..self.layers[i + 1].len() {
                out_layer.push(0.0);
            }
            for j in 0..in_layer.len() {
                let tmp = self.layers[i][j].fire(in_layer[j]);
                for x in 0..tmp.len() {
                    out_layer[x] += tmp[x];
                }
            }
            in_layer = out_layer.clone();
        }
        for i in 0..in_layer.len() {
            result.push(self.layers[self.layers.len() - 1][i].activation(in_layer[i]));
        }
        result
    }

    pub fn calculate_error(&self, training_set: &Vec<iris::Iris>) -> f64 {
        let mut error_value: f64 = 0.0;
        for i in 0..training_set.len() {
            let result = self.feed_forward(training_set[i].get_vec().clone());
            for j in 0..3 {
                error_value += pow(result[j] - training_set[i].class_num()[j], 2.0);
            }
        }
        error_value
    }
}
