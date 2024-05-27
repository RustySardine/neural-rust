use core::f32;

use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Network {
    layers: Vec<Layer>,
}
#[derive(Debug)]
struct Layer {
    weights_list: Vec<Weights>,
}
#[derive(Debug)]
struct Weights {
    data: Vec<f64>,
}
impl Network {
    fn create(n_of_inputs: usize, n_of_hidden: usize, n_of_outputs: usize) -> Self {
        let mut layers: Vec<Layer> = Vec::new();
        // Create hidden layer
        let mut hidden_layer = Layer {
            weights_list: Vec::new(),
        };
        for _neuron in 0..n_of_hidden {
            hidden_layer
                .weights_list
                .push(Weights::new(n_of_inputs + 1));
        }
        layers.push(hidden_layer);
        // Create output layer
        let mut output_layer = Layer {
            weights_list: Vec::new(),
        };
        for _neuron in 0..n_of_outputs {
            output_layer
                .weights_list
                .push(Weights::new(n_of_hidden + 1));
        }
        layers.push(output_layer);
        Self { layers }
    }
}
impl Weights {
    fn new(n_of_weights: usize) -> Self {
        let mut weights = Vec::new();
        let mut rng = thread_rng();
        for _weight in 0..n_of_weights {
            weights.push(rng.gen_range(0.0..1.0));
        }
        Self { data: weights }
    }
}

fn main() {
    let network = Network::create(2, 1, 2);
    println!("{:#?}", network);
}
