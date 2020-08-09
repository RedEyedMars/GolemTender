use packed_simd::f32x16;
use rand::Rng;

const ONES: f32x16 = f32x16::splat(1.0);
const ETA: f32x16 = f32x16::splat(0.25);

pub struct Cluster {
    pub activation: f32x16,
    pub bias: Option<f32x16>,
    pub weights: Vec<f32x16>,
    pub new_weights: Vec<f32x16>,
    pub delta: f32x16,
}

fn grad(x: f32x16) -> f32x16 {
    // ex (ex + x + 1) /(ex + 1)2
    let e = x.exp();
    let r = e * (e + x + ONES) / (e + ONES);
    r * r
}

fn error(expected: f32x16, actual: f32x16) -> f32x16 {
    actual - expected
}
fn delta_output(expected: f32x16, actual: f32x16) -> f32x16 {
    ETA * grad(error(expected, actual)) * error(expected, actual)
}
fn delta(from: f32x16, delta: f32x16) -> f32x16 {
    ETA * grad(from) * delta
}

fn reweight(weight: f32x16, delta: f32x16) -> f32x16 {
    weight - delta * weight
}

impl Cluster {
    pub fn layer<R: Rng + ?Sized>(
        (rng, low, high): (&mut R, f32, f32),
        weights: Vec<f32x16>,
    ) -> Cluster {
        Cluster {
            activation: f32x16::splat(0f32),
            bias: Some(f32x16::new(
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
            )),
            weights,
            new_weights: Vec::new(),
            delta: f32x16::splat(0f32),
        }
    }
    pub fn input<R: Rng + ?Sized>(
        (rng, low, high): (&mut R, f32, f32),
        weights: Vec<f32x16>,
    ) -> Cluster {
        Cluster {
            activation: f32x16::splat(0f32),
            bias: Some(f32x16::new(
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
                rng.gen_range(low, high),
            )),
            weights,
            new_weights: Vec::new(),
            delta: f32x16::splat(0f32),
        }
    }
    pub fn output() -> Cluster {
        Cluster {
            activation: f32x16::splat(0f32),
            bias: None,
            weights: Vec::new(),
            new_weights: Vec::new(),
            delta: f32x16::splat(0f32),
        }
    }
    pub fn random_weights<R: Rng + ?Sized>((rng, low, high): (&mut R, f32, f32)) -> f32x16 {
        f32x16::new(
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
            rng.gen_range(low, high),
        )
    }

    pub fn reset_activation(&mut self) {
        self.activation = f32x16::splat(0f32);
    }

    pub fn act(&mut self, back_layer: &Vec<Cluster>, index: usize) {
        for b in back_layer.iter() {
            self.activation += b.activation * (*b.weights.get(index).unwrap()) + b.bias.unwrap();
        }
        let x = self.activation.clone();
        let e = self.activation.exp().recpre() + ONES;
        self.activation = x / e;
    }
    pub fn react_output(&self, back_layer: &mut Vec<Cluster>, index: usize, expected: f32x16) {
        for back_node in back_layer.iter_mut() {
            back_node.delta = delta_output(self.activation, expected);
            back_node.new_weights.push(reweight(
                *back_node.weights.get(index).unwrap(),
                back_node.delta,
            ));
        }
    }
    pub fn react(&self, back_layer: &mut Vec<Cluster>, index: usize) {
        for back_node in back_layer.iter_mut() {
            back_node.delta = delta(self.activation, self.delta);
            back_node.new_weights.push(reweight(
                *back_node.weights.get(index).unwrap(),
                back_node.delta,
            ));
        }
    }
    pub fn display(&self) {
        print!("{:?},", self.activation);
    }
    pub fn activate(&mut self, activation: f32x16) {
        self.activation = activation;
    }
}

pub struct Net {
    input: Vec<Cluster>,
    layer_1: Vec<Cluster>,
    layer_2: Vec<Cluster>,
    layer_3: Vec<Cluster>,
    output: Vec<Cluster>,
}

impl Net {
    pub fn input(&mut self, activations: Vec<f32x16>) {
        for (i, l) in self.input.iter_mut().enumerate() {
            l.activate(activations.get(i).unwrap().clone());
        }
    }
    pub fn display_full(&self) {
        for l in self.input.iter() {
            l.display();
        }
        println!("\nLayer 1");
        for l in self.layer_1.iter() {
            l.display();
        }
        println!("\nLayer 2");
        for l in self.layer_2.iter() {
            l.display();
        }
        println!("\nLayer 3");
        for l in self.layer_3.iter() {
            l.display();
        }
        println!("\nOutput");
        for l in self.output.iter() {
            l.display();
        }
        println!("");
    }
    pub fn reset_activation(&mut self) {
        for l in self.input.iter_mut() {
            l.reset_activation();
        }
        for l in self.layer_1.iter_mut() {
            l.reset_activation();
        }
        for l in self.layer_2.iter_mut() {
            l.reset_activation();
        }
        for l in self.layer_3.iter_mut() {
            l.reset_activation();
        }
        for l in self.output.iter_mut() {
            l.reset_activation();
        }
    }
    pub fn reset_new_weights(&mut self) {
        for l in self.layer_3.iter_mut() {
            l.new_weights = Vec::with_capacity(self.output.len());
        }
        for l in self.layer_2.iter_mut() {
            l.new_weights = Vec::with_capacity(self.layer_3.len());
        }
        for l in self.layer_1.iter_mut() {
            l.new_weights = Vec::with_capacity(self.layer_2.len());
        }
        for l in self.input.iter_mut() {
            l.new_weights = Vec::with_capacity(self.layer_1.len());
        }
    }
    pub fn swap_weights_for_fresh(&mut self) {
        for l in self.layer_3.iter_mut() {
            l.weights = l.new_weights.clone();
        }
        for l in self.layer_2.iter_mut() {
            l.weights = l.new_weights.clone();
        }
        for l in self.layer_1.iter_mut() {
            l.weights = l.new_weights.clone();
        }
        for l in self.input.iter_mut() {
            l.weights = l.new_weights.clone();
        }
    }
    pub fn push(&mut self) {
        for (i, l) in self.layer_1.iter_mut().enumerate() {
            l.act(&self.input, i);
        }
        for (i, l) in self.layer_2.iter_mut().enumerate() {
            l.act(&self.layer_1, i);
        }
        for (i, l) in self.layer_3.iter_mut().enumerate() {
            l.act(&self.layer_2, i);
        }
        for (i, o) in self.output.iter_mut().enumerate() {
            o.act(&self.layer_3, i);
        }
    }
    pub fn pull(&mut self, expected: Vec<f32x16>) {
        /*
        self.reset_new_weights();
        for (i, o) in self.output.iter().enumerate() {
            o.react_output(&mut self.layer_3, i, *expected.get(i).unwrap());
        }
        for (i, l) in self.layer_3.iter().enumerate() {
            l.react(&mut self.layer_2.get_mut(i).unwrap(), i);
        }
        for (i, l) in self.layer_2.iter().enumerate() {
            l.react(&mut self.layer_1.get_mut(i).unwrap(), i);
        }
        for (i, l) in self.layer_1.iter().enumerate() {
            l.react(&mut self.input.get_mut(i).unwrap(), i);
        }
        self.swap_weights_for_fresh();
        */
    }
    pub fn new(input_width: usize, output_width: usize, widths: Vec<usize>) -> Net {
        let (low, high) = (-1f32, 1f32);
        let mut rng = rand::thread_rng();

        let mut input = Vec::new();
        let mut output = Vec::new();
        let mut layer_1 = Vec::new();
        let mut layer_2 = Vec::new();
        let mut layer_3 = Vec::new();

        let iw = match input_width % 16 {
            0 => input_width,
            x => input_width + (16 - x),
        }; // only allow multiples of 16
        let first_width = *widths.get(0).unwrap();
        let inw = match first_width % 16 {
            0 => first_width,
            x => first_width + (16 - x),
        };
        for _ in 0..iw / 16 {
            let mut weights = Vec::new();
            for _ in 0..inw / 16 {
                weights.push(Cluster::random_weights((&mut rng, low, high)));
            }
            input.push(Cluster::input((&mut rng, low, high), weights));
        }
        let ow = match output_width % 16 {
            0 => output_width,
            x => output_width + (16 - x),
        };
        for _ in 0..ow / 16 {
            output.push(Cluster::output());
        }
        for layer_index in 0..widths.len() {
            let width = widths.get(layer_index).unwrap();
            let next_width = match layer_index == widths.len() - 1 {
                true => output_width,
                false => *widths.get(layer_index + 1).unwrap(),
            };
            let w = match width % 16 {
                0 => *width,
                x => *width + (16 - x),
            }; // only allow multiples of 16
            let nw = match next_width % 16 {
                0 => next_width,
                x => next_width + (16 - x),
            }; // only allow multiples of 16
            for _ in 0..w / 16 {
                let mut weights = Vec::new();
                for _ in 0..nw / 16 {
                    weights.push(Cluster::random_weights((&mut rng, low, high)));
                }
                match layer_index {
                    0 => layer_1.push(Cluster::layer((&mut rng, low, high), weights)),
                    1 => layer_2.push(Cluster::layer((&mut rng, low, high), weights)),
                    2 => layer_3.push(Cluster::layer((&mut rng, low, high), weights)),
                    _ => panic!("max 3 hidden layers!"),
                };
            }
        }
        Net {
            input,
            layer_1,
            layer_2,
            layer_3,
            output,
        }
    }
}
