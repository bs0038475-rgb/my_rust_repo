fn main() {
 let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let places =  2;
  let padding =0;

  let shifted: Vec<i32> = std::iter::repeat(padding)
    .take(places)
    .chain(numbers.iter().copied())
    .take(numbers.len())
    .collect();
    println!("{:?}", shifted);
}
