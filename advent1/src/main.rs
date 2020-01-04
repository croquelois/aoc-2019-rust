use std::fs;

fn q1(filename: String) -> i32 {
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let lines = contents.lines();
  let mut sum = 0;
  for line in lines {
    sum += ((line.parse::<f64>().unwrap()/3.0).floor() - 2.0) as i32;
  }
  sum
}

fn q2(filename: String) -> i32 {
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let lines = contents.lines();
  let mut sum = 0;
  for line in lines {
    let mut mass = line.parse::<f64>().unwrap();
    loop {
      let fuel = (mass/3.0).floor() - 2.0;
      if fuel <= 0.0 { break; }
      sum += fuel as i32;
      mass = fuel;
    }
    
  }
  sum
}

fn main() {
    println!("{}", q1(String::from("data.txt")).to_string());
    println!("{}", q2(String::from("data.txt")).to_string());
}