fn split(sum: i32) -> (i32, i32) {
    let split_x = sum * 4 / 9;
    let split_y = sum - split_x;
    (split_x, split_y)
}

fn main() {
    let (main_x, main_y) = split(17);
    println!("x: {}, y: {}", main_x, main_y);
}