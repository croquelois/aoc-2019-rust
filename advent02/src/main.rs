use std::fs;

fn parse(filename: impl AsRef<std::path::Path>) -> Vec<usize> {
  return fs::read_to_string(filename).expect("Something went wrong reading the file")
    .split(",").map(|s| s.parse::<usize>().unwrap()).collect();
}

fn process(mem: &mut [usize]) {
  let mut i = 0;
  loop {
    let op = mem[i];
    let in1 = mem[i+1];
    let in2 = mem[i+2];
    let out1 = mem[i+3];
    match op {
      1 => mem[out1] = mem[in1] + mem[in2],
      2 => mem[out1] = mem[in1] * mem[in2],
      99 => break,
      _ => panic!("unexpected operator"),
    }
    i = i + 4;
  }
}

fn oracle(mem: &[usize], noun: usize, verb: usize) -> usize {
  let mut copy_mem = mem.to_vec();
  copy_mem[1] = noun;
  copy_mem[2] = verb;
  process(&mut copy_mem);
  return copy_mem[0];
}

fn q1(noun: usize, verb: usize) -> usize {
  let mut mem = parse("data.txt");
  mem[1] = noun;
  mem[2] = verb;
  process(&mut mem);
  return mem[0];
}

fn q2(objectif: usize) -> usize {
  let mem = parse("data.txt");
  for noun in 0..99 {
    for verb in 0..99 {
      if oracle(&mem, noun, verb) == objectif {
        return 100 * noun + verb;
      }
    }
  }
  panic!("no correct solution found")
}

fn main() {
    println!("Question1: {}", q1(12, 2));
    println!("Question1: {}", q2(19690720));
}

#[test]
fn test1() {
  let mut mem = parse("test1.txt");
  process(&mut mem);
  assert_eq!(mem[0], 3500);
}
