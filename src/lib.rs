use rand::{Rng, RngCore};

// NUMBERS: 0..10;
// LOWER: 10..36;
// UPPER: 36..62;
// SPECIALS: 62..94;
const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn pass_gen(buffer: &mut [u8], rng: &mut impl RngCore) {
    let len = buffer.len();
    assert!(len >= 4, "Password must be at least 4 characters long!");
    for letter in buffer.iter_mut() {
        *letter = sample(rng, 94, 0);
    }
    let mut random_indices = Vec::with_capacity(4);
    for j in len - 4..len {
        let random_index = rng.gen_range(0..=j);
        let (index, element) = random_indices
            .iter()
            .position(|index| *index == random_index)
            .map(|index| (index, j))
            .unwrap_or((random_indices.len(), random_index));
        random_indices.insert(index, element);
    }
    buffer[random_indices[0]] = sample(rng, 10, 00);
    buffer[random_indices[1]] = sample(rng, 26, 10);
    buffer[random_indices[2]] = sample(rng, 26, 36);
    buffer[random_indices[3]] = sample(rng, 32, 62);
}

#[inline(always)]
fn sample(rng: &mut impl RngCore, length: usize, offset: usize) -> u8 {
    CHARS[(rng.gen::<usize>() % length) + offset]
}
