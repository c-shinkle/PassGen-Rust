use pass_gen::pass_gen;
use rand::thread_rng;
use std::io::{stdout, Write};

fn main() {
    let mut buffer = [0; 15];
    for _ in 0..1 {
        pass_gen(&mut buffer, &mut thread_rng());
        let mut out = stdout();
        out.write_all(&buffer).unwrap();
        out.write_all(&[b'\n'; 1]).unwrap();
        out.flush().unwrap();
    }
}
