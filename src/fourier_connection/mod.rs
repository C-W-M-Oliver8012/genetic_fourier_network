use rand::prelude::*;

#[derive(Clone)]
pub struct FourierConnection {
    pub input_x: usize,
    pub input_y: usize,
    pub output_x: usize,
    pub output_y: usize,
    pub node_l: f64,
    pub weight: f64,
}

pub fn new_rand(
    input_x: usize,
    input_y: usize,
    output_x: usize,
    output_y: usize,
    node_l: f64,
) -> FourierConnection {
    FourierConnection {
        input_x,
        input_y,
        output_x,
        output_y,
        node_l,
        weight: thread_rng().gen::<f64>() * 2.0 - 1.0,
    }
}

pub fn new_zeros(
    input_x: usize,
    input_y: usize,
    output_x: usize,
    output_y: usize,
    node_l: f64,
) -> FourierConnection {
    FourierConnection {
        input_x,
        input_y,
        output_x,
        output_y,
        node_l,
        weight: 0.0,
    }
}

pub fn feedforward(fourier_connection: &FourierConnection, input: f64) -> f64 {
    (input.min(fourier_connection.node_l)).max(fourier_connection.node_l * -1.0)
        * fourier_connection.weight
}
