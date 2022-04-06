use crate::modules::{iris, network, particle};
use std::vec::Vec;

pub struct PSO {
    particles: Vec<particle::Particle>,
    xbg: Vec<f64>,
    xbg_error: f64,
}

impl PSO {
    pub fn new() -> Self {
        PSO {
            particles: Vec::new(),
            xbg: Vec::new(),
            xbg_error: 0.0,
        }
    }

    pub fn run(
        &mut self,
        t: i64,
        n: i64,
        phi1: f64,
        phi2: f64,
        network: &mut network::Network,
        dataset: &mut Vec<iris::Iris>,
    ) -> Vec<f64> {
        let mut errors: Vec<f64> = Vec::new();
        let synapses_count = network.count_synapses();
        for i in 0..n {
            self.particles.push(particle::Particle::new(synapses_count));
            self.particles[i as usize].update_xbl_error(&dataset, network);
            errors.push(self.particles[i as usize].get_xbl_error());
        }
        let mut lowest_error_index = 0;
        for j in 1..n {
            if errors[j as usize] < errors[lowest_error_index as usize] {
                lowest_error_index = j;
            }
        }
        self.xbg = self.particles[lowest_error_index as usize].get_position().clone();
        self.xbg_error = self.particles[lowest_error_index as usize].get_xbl_error();
        for _i in 0..t {
            errors.clear();
            for j in 0..n {
                self.particles[j as usize].mv(1.0, phi1, phi2, &self.xbg);
                self.particles[j as usize].check_best_pos(&dataset, network);
                network.load_weights(self.particles[j as usize].get_position().clone());
                errors.push(network.calculate_error(&dataset));
            }
            let mut lowest_error_index = 0;
            for j in 1..n {
                if errors[j as usize] < errors[lowest_error_index as usize] {
                    lowest_error_index = j;
                }
            }
            if errors[lowest_error_index as usize] < self.xbg_error {
                self.xbg = self.particles[lowest_error_index as usize].get_xbl().clone();
                self.xbg_error = self.particles[lowest_error_index as usize].get_xbl_error();
            }
        }
        self.xbg.clone()
    }
}
