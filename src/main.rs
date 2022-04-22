use pass_gen::pass_gen::pass_gen;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    for _ in 0..1000 {
        println!("{}", pass_gen(1000, &mut rng));
    }
}
