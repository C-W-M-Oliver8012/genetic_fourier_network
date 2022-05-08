pub mod fourier_connection;
pub mod fourier_network;
pub mod fourier_node;

fn main() {
    let node = fourier_node::new_rand(20, 5.0);
    fourier_node::print(&node);

    for i in -50..50 {
        println!(
            "{}: {}",
            i as f64 * 0.1,
            fourier_node::feedforward(&node, i as f64 * 0.1)
        );
    }
}
