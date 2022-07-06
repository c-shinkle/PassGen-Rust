use pass_gen::pass_gen;
use rand::thread_rng;

fn main() {
    println!("{}", pass_gen(15, &mut thread_rng()));
}
