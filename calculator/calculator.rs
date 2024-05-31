use std::io::{stdin, stdout, Write};
  
fn main(){
  println!("Enter first number: ");
  let mut num_1 = String::new();
  stdin().read_line(&mut num_1).expect("Failed to read");
  let num_1: f32 = num_1.trim().parse().expect("Failed to parse");

  println!("Enter second number: ");
  let mut num_2 = String::new();
  stdin().read_line(&mut num_2).expect("Failed to read");
  let num_2: f32 = num_2.trim().parse().expect("Failed to parse");

  println!("Enter operator: ");
  let mut operator = String::new();
  stdin().read_line(&mut operator).expect("Failed to read");
  let operator = operator.trim();

  let result = match operator {
    "+" => num_1 + num_2,
    "-" => num_1 - num_2,
    "*" => num_1 * num_2,
    "/" => num_1 / num_2,
    _ => {
      println!("Invalid operator");
      0.0
    }
  };

  println!("Result: {}", result);
}
