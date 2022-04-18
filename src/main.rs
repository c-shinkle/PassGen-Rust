use pass_gen::pass_gen::pass_gen;

fn main() {
    let password = pass_gen(15);

    println!("{:?}", password);
}
