use super::matrix::Matrix;

pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
}

impl Network {
    pub fn new(layers: Vec<usize>) -> Network {
        let mut weights = vec![];
        let mut biases= vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::random(layers[i + 1], layers[i]));
            biases.push(Matrix::random(layers[i+1], 1));
        }

        Network{
            layers,
            weights,
            biases,
            data: vec![],
        }
    }

    
}