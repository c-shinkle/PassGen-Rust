use rand::{Rng, RngCore};

// NUMBERS: 0..10;
// LOWER: 10..36;
// UPPER: 36..62;
// SPECIALS: 62..94;
const LETTERS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn pass_gen(buffer: &mut [u8], rng: &mut impl RngCore) {
    assert!(buffer.len() as u32 >= 4);
    for letter in buffer.iter_mut() {
        *letter = sample(rng, 94, 0);
    }
    let random_indexes = get_random_indexes(buffer.len() as u32, rng);
    buffer[random_indexes[0] as usize] = sample(rng, 10, 00);
    buffer[random_indexes[1] as usize] = sample(rng, 26, 10);
    buffer[random_indexes[2] as usize] = sample(rng, 26, 36);
    buffer[random_indexes[3] as usize] = sample(rng, 32, 62);
}

#[inline(always)]
fn sample(rng: &mut impl RngCore, length: u32, offset: u32) -> u8 {
    LETTERS[((rng.gen::<u32>() % length) + offset) as usize]
}

fn get_random_indexes(len: u32, rng: &mut impl RngCore) -> [u32; 4] {
    assert!(len >= 4);
    let mut indexes = Vec::with_capacity(4);
    for j in len - 4..len {
        let random_index = rng.gen_range(0..=j);
        if let Some(index) = indexes.iter().position(|index| *index == random_index) {
            indexes.insert(index, j);
        } else {
            indexes.push(random_index);
        }
    }
    indexes.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use crate::get_random_indexes;

    #[test]
    fn should_insert_with_duplicates() {
        let mut rng = StdRng::seed_from_u64(0);
        let actual = get_random_indexes(4, &mut rng);
        assert_eq!(actual, [0, 3, 1, 2]);
    }
}
