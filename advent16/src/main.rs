use std::fs;

fn phase(i : usize, j: usize, offset: usize) -> i32 {
  return match ((j + offset) / (i + 1)) % 4 {
    0 => 0,
    1 => 1,
    2 => 0,
    3 => -1,
    _ => panic!("I use modulo, it can't happen")
  }
}

fn compute(input : Vec<i32>, offset: usize) -> Vec<i32> {
  let mut output = Vec::new();
  let n = input.len();
  for i in 0..n {
    let mut sum = 0;
    for j in 0..n {
      sum = sum + input[j] * phase(i+offset,j, offset+1);
    }
    output.push(sum.abs() % 10);
  }
  return output;
}

fn compute_hack(input : Vec<i32>) -> Vec<i32> {
  let mut output = Vec::new();
  let n = input.len();
  let mut sum = 0;
  for i in 0..n {
    sum = sum + input[i];
  }
  for i in 0..n {
    output.push(sum.abs() % 10);
    sum = sum - input[i];
  }
  return output;
}

fn parse(filename: impl AsRef<std::path::Path>) -> Vec<i32> {
  let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
  return data.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect();
}

fn q1(filename: impl AsRef<std::path::Path>, nb_iter : usize) -> String {
  let mut data = parse(filename);
  for _ in 0..nb_iter {
    data = compute(data,0);
  }
  return data.iter().map(|i| i.to_string()).collect::<String>()[..8].to_string();
}

fn q2(filename: impl AsRef<std::path::Path>, nb_iter : usize) -> String {
  let data_one = parse(filename);
  let offset = data_one.iter().map(|i| i.to_string()).collect::<String>()[..7].to_string().parse::<usize>().unwrap();
  let mut data : Vec<i32> = Vec::new();
  for _ in 0..10000 {
    data.append(&mut data_one.to_vec());
  }
  println!("Length: {} Offset: {}", data.len(), offset);
  data = data.split_off(offset);
  //println!("Construction of the initial input done");
  for step in 0..nb_iter {
    data = compute_hack(data);
    //println!("Step: {}", step);
  }
  return data.iter().map(|i| i.to_string()).collect::<String>()[..8].to_string();
}

fn main() {
  println!("Question1: {}", q1("data.txt",100));
  println!("Question2: {}", q2("data.txt",100));
}

#[test]
  fn test_q1_examples1() {
  assert_eq!(q1("test1.txt", 4), "01029498");
}

#[test]
  fn test_q1_examples2() {
  assert_eq!(q1("test2.txt", 100), "24176176");
}

#[test]
  fn test_q1_examples3() {
  assert_eq!(q1("test3.txt", 100), "73745418");
}

#[test]
  fn test_q1_examples4() {
  assert_eq!(q1("test4.txt", 100), "52432133");
}

#[test]
  fn test_q2_examples1() {
  assert_eq!(q2("test5.txt", 100), "84462026");
}

#[test]
  fn test_q2_examples2() {
  assert_eq!(q2("test6.txt", 100), "78725270");
}

#[test]
  fn test_q2_examples3() {
  assert_eq!(q2("test7.txt", 100), "53553731");
}