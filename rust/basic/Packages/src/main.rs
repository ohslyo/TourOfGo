use rand::Rng;

fn main() {
    let num: i32 = rand::rng().random_range(0..=10);
    println!("My favorite number is, {}", num);
}