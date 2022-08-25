use std::io;

fn main() {
  let mut input = String::new();

  id::stdin().read_line(&mut inpu);
  println!("Input: {}", input);
  let mars_weight = calculate_mars_weight(100.0);
  println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_mars_weight(weight: f32) -> f32 {
  (weight / 9.81) * 3.711 
}