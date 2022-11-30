use std::io::{stdout, Write};

use rand::{Rng, RngCore};
use rand::prelude::SmallRng;
use rand::SeedableRng;

// NUMBERS: 0..10;
// LOWER: 10..36;
// UPPER: 36..62;
// SPECIALS: 62..94;
const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn pass_gen(buffer: &mut [u8], rng: &mut impl RngCore) {
    let len = buffer.len();
    assert!(len >= 4);
    for letter in buffer.iter_mut() {
        *letter = sample(rng, 94, 0);
    }
    let mut random_indices = Vec::with_capacity(4);
    for j in len - 4..len {
        let random_index = rng.gen_range(0..=j);
        if let Some(index) = random_indices.iter().position(|index| *index == random_index) {
            random_indices.insert(index, j);
        } else {
            random_indices.push(random_index);
        }
    }
    buffer[random_indices[0]] = sample(rng, 10, 00);
    buffer[random_indices[1]] = sample(rng, 26, 10);
    buffer[random_indices[2]] = sample(rng, 26, 36);
    buffer[random_indices[3]] = sample(rng, 32, 62);
}

#[inline(always)]
fn sample(rng: &mut impl RngCore, length: usize, offset: usize) -> u8 {
    CHARS[((rng.gen::<usize>() % length) + offset)]
}

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
