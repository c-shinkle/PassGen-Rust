pub mod pass_gen {
    use rand::prelude::{SliceRandom, ThreadRng};

    const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    const LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    const UPPER: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    const SPECIALS: [char; 32] = [
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
        '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
    ];
    const ALL: [char; 94] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*',
        '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`',
        '{', '|', '}', '~',
    ];

    pub fn pass_gen(size: usize, rng: &mut ThreadRng) -> String {
        let indices: Vec<usize> = (0..size)
            .collect::<Vec<usize>>()
            .choose_multiple(rng, 4)
            .cloned()
            .collect();
        let mut password: Vec<u8> = vec![0; size];
        password[indices[0]] = sample(rng, &NUMBERS);
        password[indices[1]] = sample(rng, &LOWER);
        password[indices[2]] = sample(rng, &UPPER);
        password[indices[3]] = sample(rng, &SPECIALS);
        for letter in password.iter_mut() {
            if *letter == 0 {
                *letter = sample(rng, &ALL);
            }
        }
        unsafe { String::from_utf8_unchecked(password) }
    }

    fn sample(rng: &mut ThreadRng, data: &[char]) -> u8 {
        *data.choose(rng).unwrap() as u8
    }
}
