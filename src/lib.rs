use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::iter::FromIterator;

// NUMBERS: 0..10;
// LOWER: 10..36;
// UPPER: 36..62;
// SPECIALS: 62..94;
const LETTERS: [char; 94] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.',
    '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

pub fn pass_gen(size: u32, rng: &mut ThreadRng) -> String {
    let non_random_indexes = Vec::from_iter(0..size);
    let random_indexes: Vec<&u32> = non_random_indexes.choose_multiple(rng, 4).collect();
    let mut password: Vec<u8> = vec![0; size as usize];
    password[*random_indexes[0] as usize] = sample(rng, 10, 00);
    password[*random_indexes[1] as usize] = sample(rng, 26, 10);
    password[*random_indexes[2] as usize] = sample(rng, 26, 36);
    password[*random_indexes[3] as usize] = sample(rng, 32, 62);
    for letter in password.iter_mut() {
        if *letter == 0 {
            *letter = sample(rng, 94, 0);
        }
    }
    unsafe { String::from_utf8_unchecked(password) }
}

fn sample(rng: &mut ThreadRng, length: u32, offset: u32) -> u8 {
    LETTERS[((rng.gen::<u32>() % length) + offset) as usize] as u8
}
