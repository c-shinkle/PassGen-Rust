#![feature(test)]

extern crate rand;
extern crate test;

#[cfg(test)]
mod tests {
    use pass_gen::pass_gen;
    use rand::thread_rng;
    use test::Bencher;

    #[bench]
    fn thousand_passwords_thousand_letters(b: &mut Bencher) {
        let mut rng = thread_rng();
        let mut buffer = [0; 1000];
        b.iter(|| {
            for _ in 0..1000 {
                pass_gen(&mut buffer, &mut rng);
            }
        });
    }
}
