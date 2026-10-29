fn main() {
    let x = vec![1, 2, 3, 4, 5];
    let y = vec![2, 4, 6, 8, 10];
    let depth = x[4] * y[4] / 2;
    println!("depth = {}", depth);
}
fn  calcDepth(x: i32, y: i32) -> i32 {
    return x * y / 2;
}
calcDepth(1,2);