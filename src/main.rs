mod modules;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn index_max(tab: Vec<f64>) -> i32 {
    let mut max_value = tab[0];
    let mut max = 0;
    for i in 1..tab.len() {
        if max_value < tab[i] {
            max_value = tab[i];
            max = i;
        }
    }
    max as i32
}

fn shuffle(tab: &mut Vec<modules::iris::Iris>) {
    tab.shuffle(&mut thread_rng());
}

fn main() {
    let mut input_data = match modules::reader::read_from_file("./data/iris.csv") {
        Ok(res) => res,
        Err(_e) => Vec::new(),
    };
    shuffle(&mut input_data);
    let mut training_data: Vec<modules::iris::Iris> = Vec::new();
    let mut validation_data: Vec<modules::iris::Iris> = Vec::new();
    let split: usize = (0.7 * (input_data.len() as f64)).round() as usize;
    for i in 0..input_data.len() {
        if i < split {
            training_data.push(input_data[i as usize].clone());
        } else {
            validation_data.push(input_data[i as usize].clone());
        }
    }
    let mut neural_network: modules::network::Network = modules::network::Network::new();
    neural_network.add_layer(4);
    neural_network.add_layer(5);
    neural_network.add_layer(3);
    let mut pso: modules::pso::PSO = modules::pso::PSO::new();
    let weights = pso.run(100, 10, 0.7, 0.3, &mut neural_network, &mut training_data);
    neural_network.load_weights(weights);
    let mut correct: i64 = 0;
    for i in 0..validation_data.len() {
        let result = index_max(neural_network.feed_forward(validation_data[i].get_vec()));
        let expected = index_max(validation_data[i].class_num());
        if result == expected {
            correct += 1;
        }
    }
    println!(
        "Accuracy: {} %",
        (100.0) * (correct as f64) / (validation_data.len() as f64)
    );
}
