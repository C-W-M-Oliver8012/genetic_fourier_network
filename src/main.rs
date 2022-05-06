pub mod fourier_node;

fn main() {
    let node = fourier_node::new_rand(10, 10.0);
    fourier_node::print(&node);
}
