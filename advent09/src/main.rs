use std::fs;
//use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct State {
  ip: usize, // instruction pointer
  rb: i64, // relative base
  mem: HashMap<usize,i64>,
  finished: bool,
}

fn parse_string(data: &String) -> Vec<i64> {
  return data.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
}

fn parse(filename: impl AsRef<std::path::Path>) -> Vec<i64> {
  return parse_string(&fs::read_to_string(filename).expect("Something went wrong reading the file"));
}

impl State {
  fn new_from_vector(code: &Vec<i64>) -> State {
    let mut mem = HashMap::new();
    for (pos, elem) in code.iter().enumerate() {
      mem.insert(pos, *elem);
    }
    return State {
      ip: 0,
      rb: 0,
      mem: mem,
      finished: false
    }
  }

  fn new_from_string(data: &String) -> State {
    return State::new_from_vector(&parse_string(data));
  }

  fn new_from_file(filename: impl AsRef<std::path::Path>) -> State {
    return State::new_from_vector(&parse(filename));
  }

  fn get_mem(&self, pos: usize) -> i64 {
    return *self.mem.get(&pos).unwrap_or(&0);
  }

  fn set_mem_indirect(&mut self, pos: usize, v: i64) {
    self.mem.insert(self.get_mem(pos) as usize, v);
  }

  fn get_param(&self, narg: usize) -> i64 {
    let mode = (self.get_mem(self.ip)/10i64.pow(narg as u32 + 2))%10;
    let val = self.get_mem(self.ip+1+narg);
    return match mode { 
      0 => self.get_mem(val as usize),
      1 => val,
      2 => self.get_mem((self.rb + val) as usize),
      _ => panic!("unexpected operator"),
    }
  }

  fn set_param(&mut self, narg: usize, val: i64) {
    let mode = (self.get_mem(self.ip)/10i64.pow(narg as u32 + 2))%10;
    let pos = self.get_mem(self.ip+1+narg);
    self.mem.insert(match mode { 
      0 => pos,
      1 => panic!("mode immediate cannot be used for a output parameter"),
      2 => self.rb + pos,
      _ => panic!("unexpected operator"),
    } as usize, val);
  }

  fn inc_ip(&mut self, v: i64) {
    self.ip = self.ip + v as usize;
  }
  
  fn process(&mut self, input: &mut VecDeque<i64>) -> VecDeque<i64> {
    let mut output : VecDeque<i64> = VecDeque::new();
    loop {
      //println!("{:?}", self);
      match self.get_mem(self.ip)%100 {
        1 => {
          self.set_param(2, self.get_param(0) + self.get_param(1));
          self.inc_ip(4);
        }
        2 => {
          self.set_param(2, self.get_param(0) * self.get_param(1));
          self.inc_ip(4);
        }
        3 => {
          if input.len() == 0 { break; }
          self.set_param(0, input.pop_front().expect("input should not be empty"));
          self.inc_ip(2);
        }
        4 => {
          output.push_back(self.get_param(0));
          self.inc_ip(2);
        }
        5 => {
          if self.get_param(0) != 0 {
            self.ip = self.get_param(1) as usize;
          } else {
            self.inc_ip(3);
          }
        }
        6 => {
          if self.get_param(0) == 0 {
            self.ip = self.get_param(1) as usize;
          } else {
            self.inc_ip(3);
          }
        }
        7 => {
          self.set_param(2, if self.get_param(0) < self.get_param(1) { 1 } else { 0 });
          self.inc_ip(4);
        }
        8 => {
          self.set_param(2, if self.get_param(0) == self.get_param(1) { 1 } else { 0 });
          self.inc_ip(4);
        }
        9 => {
          self.rb = self.rb + self.get_param(0);
          self.inc_ip(2);
        }
        99 => {
          self.finished = true;
          break;
        }
        _ => panic!("unexpected operator"),
      }
    }
    return output;
  }
}

fn q1(filename: impl AsRef<std::path::Path>) -> i64 {
  let mut state = State::new_from_file(filename);
  let mut input : VecDeque<i64> = VecDeque::new();
  input.push_back(1);
  let output = state.process(&mut input);
  return *output.iter().next().expect("should output one value");
}

fn q2(filename: impl AsRef<std::path::Path>) -> i64 {
  let mut state = State::new_from_file(filename);
  let mut input : VecDeque<i64> = VecDeque::new();
  input.push_back(2);
  let output = state.process(&mut input);
  return *output.iter().next().expect("should output one value");
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2: {}", q2("data.txt"));
}

#[test]
fn test_examples1() {
  let code = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
  let mut state = State::new_from_string(&code.to_string());
  let mut input : VecDeque<i64> = VecDeque::new();
  let output = state.process(&mut input);
  let result : Vec<String> = output.iter().map(|i| i.to_string()).collect();
  assert_eq!(result.join(","), code);
}

#[test]
fn test_examples2() {
  let code = "1102,34915192,34915192,7,4,7,99,0";
  let mut state = State::new_from_string(&code.to_string());
  let mut input : VecDeque<i64> = VecDeque::new();
  let output = state.process(&mut input);
  let result : Vec<String> = output.iter().map(|i| i.to_string()).collect();
  assert_eq!(result.join(",").len(), 16);
}

#[test]
fn test_examples3() {
  let code = "104,1125899906842624,99";
  let mut state = State::new_from_string(&code.to_string());
  let mut input : VecDeque<i64> = VecDeque::new();
  let output = state.process(&mut input);
  let result : Vec<String> = output.iter().map(|i| i.to_string()).collect();
  assert_eq!(result.join(","), "1125899906842624");
}