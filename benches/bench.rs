#![feature(test)]

extern crate rand;
extern crate test;

#[cfg(test)]
mod tests {
    use pass_gen::pass_gen;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;
    use test::Bencher;

    #[bench]
    fn thousand_passwords_thousand_letters(b: &mut Bencher) {
        let mut rng = SmallRng::from_entropy();
        let mut buffer = [0; 1000];
        b.iter(|| {
            for _ in 0..10000 {
                pass_gen(&mut buffer, &mut rng);
            }
        });
    }
}
