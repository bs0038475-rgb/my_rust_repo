fn main() {
    let x = vec![1, 2, 3, 4, 5];
    let y = vec![2, 4, 6, 8, 10];
    let depth = x[4] * y[4] / 2;
    println!("depth = {}", depth);
    for i in 0..x.len() {
        println!("{} * {} / 2 = {}", x[i], y[i], x[i] * y[i] / 2);
    }
}
