use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Network {
    layers: Vec<Layer>,
}
#[derive(Debug)]
struct Layer {
    weights_list: Vec<Weights>,
}
#[derive(Debug, Clone)]
struct Weights {
    data: Vec<f64>,
}

#[derive(Debug)]
struct Input {
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
    fn forward_propagate(&self, inputs: Input) -> Input {
        let mut output = inputs;

        for layer in &self.layers {
            let mut new_inputs = Vec::new();
            for neuron in &layer.weights_list {
                new_inputs.push(neuron.clone().activate(&output));
            }
            output.data = new_inputs;
        }
        output
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
    fn activate(&mut self, inputs: &Input) -> f64 {
        let mut activation: f64 = self.data.pop().unwrap();
        for i in 0..self.data.len() {
            activation += self.data[i] * inputs.data[i];
        }
        activation *= -1.0;
        1.0 / (1.0 + activation.exp())
    }
}

fn main() {
    let network = Network::create(2, 1, 2);
    let input = Input {
        data: vec![1.0, 0.0],
    };
    let outcome = network.forward_propagate(input);

    println!("{outcome:#?}");
}
