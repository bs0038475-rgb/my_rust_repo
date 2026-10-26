fn main() {
    println!("Hello, world!");
    let x = vec![1, 2, 3];
    let y = vec![2,4,6];
    let z = vec![3,6,9];
    let result = x[0] * y[1] * z[2];
    println!("{}", result);
    let sum = add(&x[0], &y[1], &z[2]);
    println!("{}", sum);
    for i in x {
        println!("{}", i);
    }
    for i in y {
        println!("{}", i);
    }
    for i in z {
        println!("{}", i);
    }
}

fn add(x: &i32, y: &i32, z: &i32) -> i32 {
    *x + *y + *z
}
