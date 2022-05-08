use rand::prelude::*;

#[derive(Clone)]
pub struct FourierNode {
    pub n: usize,
    pub l: f64,
    pub pi_l: f64,
    pub bias: f64,
    pub cos: Vec<f64>,
    pub sin: Vec<f64>,
}

pub fn new_rand(n: usize, l: f64) -> FourierNode {
    assert!(l != 0.0);

    let mut fourier_node = FourierNode {
        n,
        l,
        pi_l: std::f64::consts::PI / l,
        bias: thread_rng().gen::<f64>() * 2.0 - 1.0,
        cos: Vec::with_capacity(n),
        sin: Vec::with_capacity(n),
    };

    for _ in 0..n {
        fourier_node.cos.push(thread_rng().gen::<f64>() * 2.0 - 1.0);
        fourier_node.sin.push(thread_rng().gen::<f64>() * 2.0 - 1.0);
    }

    fourier_node
}

pub fn new_zeros(n: usize, l: f64) -> FourierNode {
    assert!(l != 0.0);

    let mut fourier_node = FourierNode {
        n,
        l,
        pi_l: std::f64::consts::PI / l,
        bias: 0.0,
        cos: Vec::with_capacity(n),
        sin: Vec::with_capacity(n),
    };

    for _ in 0..n {
        fourier_node.cos.push(0.0);
        fourier_node.sin.push(0.0);
    }

    fourier_node
}

pub fn print(fourier_node: &FourierNode) {
    println!("Fourier Node:");
    println!("N: {}", fourier_node.n);
    println!("L: -{} to {}", fourier_node.l, fourier_node.l,);
    println!("PI / L: {}", fourier_node.pi_l);

    print!("{} ", fourier_node.bias);
    for i in 1..fourier_node.n + 1 {
        print!(
            "+ {}*cos({}x) + {}*sin({}x) ",
            fourier_node.cos[i - 1],
            fourier_node.pi_l * i as f64,
            fourier_node.sin[i - 1],
            fourier_node.pi_l * i as f64
        );
    }
    println!();
}

pub fn feedforward(fourier_node: &FourierNode, input: f64) -> f64 {
    let mut output = fourier_node.bias;

    for i in 1..fourier_node.n + 1 {
        let k = fourier_node.pi_l * i as f64 * input;
        let sin_cos_values = k.sin_cos();

        output += fourier_node.sin[i - 1] * sin_cos_values.0;
        output += fourier_node.cos[i - 1] * sin_cos_values.1;
    }

    (output.min(fourier_node.l)).max(fourier_node.l * -1.0)
}

pub fn add(a: &FourierNode, b: &FourierNode) -> FourierNode {
    assert!(a.n == b.n);
    assert!(a.l == b.l);
    assert!(a.pi_l == b.pi_l);

    let mut c = a.clone();

    c.bias += b.bias;
    for i in 0..c.n {
        c.cos[i] += b.cos[i];
        c.sin[i] += b.sin[i];
    }

    c
}

pub fn scalar(a: &FourierNode, s: f64) -> FourierNode {
    let mut b = a.clone();

    b.bias *= s;
    for i in 0..b.n {
        b.cos[i] *= s;
        b.sin[i] *= s;
    }

    b
}
