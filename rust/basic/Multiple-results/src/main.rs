fn swap<'a>(x: &'a str, y: &'a str) -> (&'a str, &'a str) {
    (y, x)
}
fn main() {
    let (a, b) = swap("hello", "world");
    println!("{} {}", a, b);
}
