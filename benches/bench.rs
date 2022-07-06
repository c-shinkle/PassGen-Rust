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
        b.iter(|| {
            for _ in 0..1000 {
                pass_gen(1000, &mut rng);
            }
        });
    }
}
