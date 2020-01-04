#![allow(dead_code,unused_imports)]
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug)]
struct State {
  ip: usize, // instruction pointer
  rb: i64, // relative base
  mem: HashMap<usize,i64>,
  finished: bool,
}

type Point = (i64,i64);

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

  fn set_mem(&mut self, pos: usize, v: i64) {
    self.mem.insert(pos, v);
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

fn consume_output(mut output: VecDeque<i64>, queues: &mut Vec<VecDeque<i64>>) -> VecDeque<i64> {
  let mut broadcast = VecDeque::new();
  while !output.is_empty() {
    let addr = output.pop_front().unwrap() as usize;
    if addr == 255 {
      broadcast.push_back(output.pop_front().unwrap());
      broadcast.push_back(output.pop_front().unwrap());
    }else{
      queues[addr].push_back(output.pop_front().unwrap());
      queues[addr].push_back(output.pop_front().unwrap());
    }
  }
  return broadcast;
}

fn init_network(filename: impl AsRef<std::path::Path>) -> (Vec<State>, Vec<VecDeque<i64>>){
  let code = parse(filename);
  let mut computers : Vec<State> = Vec::new();
  let mut queues : Vec<VecDeque<i64>> = Vec::new();
  for ip in 0..50 {
    queues.push(VecDeque::from(vec![ip as i64]));
  }
  for ip in 0..50 {
    let mut state = State::new_from_vector(&code);
    let output = state.process(&mut queues[ip]);
    computers.push(state);
    consume_output(output, &mut queues);
  }
  return (computers, queues);
}

fn q1(filename: impl AsRef<std::path::Path>) -> i64 {
  let (mut computers, mut queues) = init_network(filename);
  loop {
    for ip in 0..50 {
      let queue = &mut queues[ip];
      if queue.is_empty() {
        queue.push_back(-1);
      }
      let output = computers[ip].process(queue);
      let mut broadcast = consume_output(output, &mut queues);
      if !broadcast.is_empty() {
        broadcast.pop_front();
        return broadcast.pop_front().unwrap();
      }
    }
  }
}

fn q2(filename: impl AsRef<std::path::Path>) -> i64 {
  let (mut computers, mut queues) = init_network(filename);
  let mut nat : Option<Point> = None;
  let mut already_delivered : HashSet<i64> = HashSet::new();
  loop {
    //println!("new step");
    for queue in &mut queues {
      if queue.is_empty() {
        queue.push_back(-1);
      }
    }
    for ip in 0..50 {
      let queue = &mut queues[ip];
      let output = computers[ip].process(queue);
      let mut nat_msg = consume_output(output, &mut queues);
      while !nat_msg.is_empty() {
        let x = nat_msg.pop_front().expect("cannot unwrap X for NAT");
        let y = nat_msg.pop_front().expect("cannot unwrap Y for NAT");
        nat = Some((x, y));
        //println!("new value for the NAT: <{},{}>", x, y);
      }
    }
    let mut all_empty = true;
    for queue in &mut queues {
      if !queue.is_empty() {
        all_empty = false;
      }
    }
    if all_empty {
      //queues[0].pop_front();
      let (x,y) = nat.expect("all are empty but NAT is empty also");
      //println!("all queues are empty, deliver <{},{}> to computer 0", x, y);
      queues[0].push_back(x);
      queues[0].push_back(y);
      if !already_delivered.insert(y) {
        return y;
      }
    }
  }
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2: {}", q2("data.txt"));
}