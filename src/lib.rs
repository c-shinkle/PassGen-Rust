pub mod pass_gen {
    use rand::{Rng, thread_rng};
    use rand::rngs::ThreadRng;

    const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    const LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    const UPPER: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    const SPECIALS: [char; 32] = ['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];
    const ALL: [char; 94] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];

    fn sample(rng: &mut ThreadRng, data: &[char]) -> u8 {
        data[rng.gen_range(0..data.len())] as u8
    }

    fn shuffle<T>(rng: &mut ThreadRng, data: &mut [T]) {
        for i in (1..data.len()).rev() {
            let j = rng.gen_range(0..(i + 1));
            data.swap(i, j);
        }
    }

    pub fn pass_gen(size: usize) -> String {
        //setup randomness and password
        let mut rng = thread_rng();
        let mut password: Vec<u8> = Vec::with_capacity(size);
        //guarantee at least one of each type of letter
        password.push(sample(&mut rng, &NUMBERS));
        password.push(sample(&mut rng, &LOWER));
        password.push(sample(&mut rng, &UPPER));
        password.push(sample(&mut rng, &SPECIALS));
        //randomly sample the rest
        for _ in 4..size {
            password.push(sample(&mut rng, &ALL));
        }
        //shuffle to guarantee random order
        shuffle(&mut rng, &mut password);
        //build (unsafe) ASCII String quickly
        return unsafe { String::from_utf8_unchecked(password) };
    }
}