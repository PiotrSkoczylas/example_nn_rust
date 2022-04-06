use crate::modules::{iris, network};
use rand::Rng;
use std::vec::Vec;

#[derive(Debug)]
pub struct Particle {
    position: Vec<f64>,
    v: Vec<f64>,
    xbl: Vec<f64>,
    xbl_error: f64,
}

impl Clone for Particle {
    fn clone(&self) -> Particle {
        let mut particle = Particle {
            position: Vec::new(),
            v: Vec::new(),
            xbl: Vec::new(),
            xbl_error: self.xbl_error,
        };
        for i in 0..self.position.len() {
            particle.position.push(self.position[i]);
            particle.v.push(self.v[i]);
            particle.xbl.push(self.xbl[i]);
        }
        particle
    }
}

impl Particle {
    pub fn new(n: i64) -> Self {
        let mut particle = Particle {
            position: Vec::new(),
            v: Vec::new(),
            xbl: Vec::new(),
            xbl_error: 0.0,
        };
        let mut rng = rand::thread_rng();
        for _i in 0..n {
            particle.position.push(rng.gen_range(0.0..1.0));
            particle.v.push(0.1);
        }
        particle.xbl = particle.position.clone();
        particle
    }
    pub fn mv(&mut self, w: f64, phi1: f64, phi2: f64, xbg: &Vec<f64>) {
        let mut rng = rand::thread_rng();
        for i in 0..self.position.len() {
            self.v[i] = w * self.v[i]
                + phi1 * rng.gen_range(0.0..1.0) * (self.xbl[i] - self.position[i])
                + phi2 * rng.gen_range(0.0..1.0) * (xbg[i] - self.position[i]);
            self.position[i] += self.v[i];
        }
    }

    pub fn update_xbl_error(&mut self, dataset: &Vec<iris::Iris>, network: &mut network::Network) {
        network.load_weights(self.xbl.clone());
        self.xbl_error = network.calculate_error(&dataset);
    }

    pub fn check_best_pos(&mut self, dataset: &Vec<iris::Iris>, network: &mut network::Network) {
        network.load_weights(self.position.clone());
        let error = network.calculate_error(&dataset);
        if error < self.xbl_error {
            self.xbl_error = error;
            self.xbl = self.position.clone();
        }
    }

    pub fn get_xbl_error(&self) -> f64 {
        self.xbl_error
    }
    pub fn get_xbl(&self) -> Vec<f64> {
        self.xbl.clone()
    }
    pub fn get_position(&self) -> Vec<f64> {
        self.position.clone()
    }
}
