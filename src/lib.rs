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
        let mut password: Vec<u8> = vec![0; size];
        //guarantee at least one of each type of letter
        let mut indices: [usize; 4] = [0, 1, 2, 3];
        shuffle(&mut rng, &mut indices);
        password[indices[0]] = sample(&mut rng, &NUMBERS);
        password[indices[1]] = sample(&mut rng, &LOWER);
        password[indices[2]] = sample(&mut rng, &UPPER);
        password[indices[3]] = sample(&mut rng, &SPECIALS);
        //randomly sample the rest
        for i in 0..size {
            if password[i] == 0 {
                password[i] = sample(&mut rng, &ALL);
            }
        }
        //build (unsafe) ASCII String quickly
        return unsafe { String::from_utf8_unchecked(password) };
    }
}