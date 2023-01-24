use pass_gen::pass_gen;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use std::io::{stdout, Write};

fn main() {
    let mut buffer = [0; 1000];
    let mut rng = SmallRng::from_entropy();
    let mut out = stdout();
    for _ in 0..10000 {
        pass_gen(&mut buffer, &mut rng);
        out.write_all(&buffer).unwrap();
        out.write_all(&[b'\n'; 1]).unwrap();
        out.flush().unwrap();
    }
}
