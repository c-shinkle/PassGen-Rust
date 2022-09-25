use pass_gen::pass_gen;
use rand::thread_rng;

fn main() {
    for _ in 0..1000 {
        println!("{}", pass_gen(1000, &mut thread_rng()));
    }
}
