use crate::fourier_connection;
use crate::fourier_node;

pub const INPUT_X: usize = 0;
pub const OUTPUT_X: usize = 1;

#[derive(Clone)]
pub struct FourierNetwork {
    pub num_inputs: usize,
    pub num_outputs: usize,
    pub node_n: usize,
    pub node_l: f64,
    pub grid: Vec<Vec<fourier_node::FourierNode>>,
    pub connections: Vec<Vec<Vec<fourier_connection::FourierConnection>>>,
}

/*
    The X grid location 0 is reserved for inputs but is never used within the grid as
    the feedforward function will take in a vector of inputs. This exists for the use of
    consistent connection locations for input data. The X grid location 1 is reserved for
    outputs. This is to simplify the fact that the grid can grow in the X direction and if
    the outputs existed at the end of the grid, we would have to move the output nodes and
    change connection information every time the grid grew in the x direction. The grid is
    a 2d vector while connections are stored in a 3d vector because every node may have multiple
    connections.
*/

pub fn new_rand(
    num_inputs: usize,
    num_outputs: usize,
    node_n: usize,
    node_l: f64,
) -> FourierNetwork {
    let mut fourier_network = FourierNetwork {
        num_inputs,
        num_outputs,
        node_n,
        node_l,
        grid: vec![vec![], vec![]],
        connections: vec![vec![Vec::with_capacity(num_inputs * num_outputs)]],
    };

    // init output nodes
    for _ in 0..num_outputs {
        fourier_network.grid[1].push(fourier_node::new_rand(node_n, node_l));
    }

    // connection all inputs to outputs

    fourier_network
}
