#![allow(dead_code)]
use std::fs;
//use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;
//use num::integer::gcd;

#[derive(Debug)]
struct State {
  ip: usize, // instruction pointer
  rb: i64, // relative base
  mem: HashMap<usize,i64>,
  finished: bool,
}

type Point = (i64,i64);

struct Grid {
  data: HashMap<Point,char>,
  dft: char
}

impl Grid {
  fn new() -> Grid {
    return Grid {
      data: HashMap::new(),
      dft: ' '
    }
  }

  fn set(&mut self, pos: Point, chr: char) {
    self.data.insert(pos, chr);    
  }

  fn get(&self, pos : &Point) -> char {
    return *self.data.get(pos).unwrap_or(&self.dft);
  }

  fn print_minmax(&self, min_x: i64, max_x: i64, min_y:i64, max_y:i64) {
    for y in min_y..(max_y+1) {
      let mut line = Vec::new();
      for x in min_x..(max_x+1) {
        line.push(self.get(&(x,y)));
      }
      let line_str : String = line.into_iter().collect();
      println!("{}", line_str);
    }
  }

  fn minmax(&self) -> (i64, i64, i64, i64) {
    let min_x = self.data.keys().fold( 99999, |a,i| a.min(i.0));
    let max_x = self.data.keys().fold(-99999, |a,i| a.max(i.0));
    let min_y = self.data.keys().fold( 99999, |a,i| a.min(i.1));
    let max_y = self.data.keys().fold(-99999, |a,i| a.max(i.1));
    return (min_x, max_x, min_y, max_y);
  }
  
  fn print(&self) {
    let (min_x, max_x, min_y, max_y) = self.minmax();
    self.print_minmax(min_x, max_x, min_y, max_y);
  }
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

fn output_to_grid(output : &VecDeque<i64>) -> (Grid, VecDeque<i64>) {
  let mut grid = Grid::new();
  let mut p : Point = (0,0);
  let mut unprocessed = VecDeque::new();
  for o in output {
    match o {
      10 => {
        p.0 = 0;
        p.1 = p.1 + 1;
      },
      _ => {
        if *o < 256 {
          grid.set(p, *o as u8 as char);
          p.0 = p.0 + 1;
        } else {
          unprocessed.push_back(*o);
        }
      }
    }
  }
  return (grid, unprocessed);
}

fn add_to_input(s : &str, input : &mut VecDeque<i64>) {
  for c in s.chars() {
    input.push_back(c as u8 as i64);
  }
  input.push_back('\n' as u8 as i64);
}

fn q1(filename: impl AsRef<std::path::Path>) -> i64 {
  let code = parse(filename);
  let mut state = State::new_from_vector(&code);
  let mut input : VecDeque<i64> = VecDeque::new();
  add_to_input("NOT C J",&mut input);
  add_to_input("AND D J",&mut input);
  add_to_input("NOT A T",&mut input);
  add_to_input("OR T J",&mut input);
  add_to_input("WALK",&mut input);
  let output = state.process(&mut input);
  let (grid, unprocessed) = output_to_grid(&output);
  grid.print();
  return unprocessed[0];
}

fn q2(filename: impl AsRef<std::path::Path>) -> i64 {
  let code = parse(filename);
  let mut state = State::new_from_vector(&code);
  let mut input : VecDeque<i64> = VecDeque::new();
  /*add_to_input("NOT H J",&mut input);
  add_to_input("AND E J",&mut input);
  add_to_input(" OR H J",&mut input);
  add_to_input("AND D J",&mut input);
  add_to_input("NOT C T",&mut input);
  add_to_input("AND T J",&mut input);
  
  add_to_input("NOT A T",&mut input);
  add_to_input( "OR T J",&mut input);
  
  add_to_input("NOT E T",&mut input);
  add_to_input("AND A T",&mut input);
  add_to_input("AND D T",&mut input);
  add_to_input( "OR T J",&mut input);*/
  
  add_to_input("NOT A J",&mut input);
  add_to_input("NOT B T",&mut input);
  add_to_input( "OR T J",&mut input);
  add_to_input("NOT C T",&mut input);
  add_to_input( "OR T J",&mut input);
  
  add_to_input("NOT H T",&mut input);
  add_to_input("AND E T",&mut input);
  add_to_input( "OR H T",&mut input);
  add_to_input("AND T J",&mut input);
  
  add_to_input("AND D J",&mut input);
  
  add_to_input("RUN",&mut input);
  let output = state.process(&mut input);
  let (grid, unprocessed) = output_to_grid(&output);
  grid.print();
  return unprocessed[0];
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2: {}", q2("data.txt"));
}
