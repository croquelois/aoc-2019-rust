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

fn is_pulled(code : &Vec<i64>, p: &Point) -> bool {
  let mut state = State::new_from_vector(code);
  let mut input : VecDeque<i64> = VecDeque::new();
  input.push_back(p.0);
  input.push_back(p.1);
  let output = state.process(&mut input);
  return *output.front().expect("Should not be empty") == 1;
}

fn scan(code: &Vec<i64>, max_x : i64, max_y : i64) -> Grid {
  let mut grid = Grid::new();
  for x in 0..max_x {
    for y in 0..max_y {
      let pulled = is_pulled(code, &(x,y));
      grid.set((x,y),match pulled { true => '#', false => '.' });
    }
  }
  return grid;
}
fn analysis_x(code: &Vec<i64>, start_x: i64, stop_gap : i64) -> i64 {
  let mut prev_y_start = 0;
  let mut prev_y_stop = 0;
  for x in start_x.. {
    let mut prev_pulled = false;
    let mut y = prev_y_start;
    loop {
      let pulled = is_pulled(code, &(x,y));
      if !prev_pulled && pulled {
        prev_y_start = y;
        prev_pulled = true;
        y = prev_y_stop.max(y);
      }
      if prev_pulled && !pulled {
        prev_y_stop = y;
        println!("At x:{} - Size {}", x, prev_y_stop-prev_y_start-1);
        if prev_y_stop-prev_y_start-1 >= stop_gap {
          return x;
        }
        //println!("At x:{} - Slope {:.2} and {:.2}", x, (prev_y_start as f64)/(x as f64), ((prev_y_stop-1) as f64)/(x as f64));
        break;
      }
      y = y + 1;
    }
  }
  panic!("Should never manage to exit through here");
}
/*
fn analysis_x(code: &Vec<i64>, start_x: i64, stop_gap : i64) -> i64 {
  let mut prev_y_start = 0;
  let mut prev_y_stop = 0;
  for x in start_x.. {
    let mut prev_pulled = false;
    let mut y = prev_y_start;
    loop {
      let pulled = is_pulled(code, &(x,y));
      if !prev_pulled && pulled {
        prev_y_start = y;
        prev_pulled = true;
        y = prev_y_stop.max(y);
      }
      if prev_pulled && !pulled {
        prev_y_stop = y;
        println!("At x:{} - Size {}", x, prev_y_stop-prev_y_start-1);
        if prev_y_stop-prev_y_start-1 >= stop_gap {
          return x;
        }
        //println!("At x:{} - Slope {:.2} and {:.2}", x, (prev_y_start as f64)/(x as f64), ((prev_y_stop-1) as f64)/(x as f64));
        break;
      }
      y = y + 1;
    }
  }
  panic!("Should never manage to exit through here");
}
*/
fn analysis(code: &Vec<i64>, dim: usize, start: i64, stop_gap : i64) -> i64 {
  let mut prev_start = 0;
  let mut prev_stop = 0;
  for step in start.. {
    let mut prev_pulled = false;
    let mut cur = prev_start;
    loop {
      let mut p : Point;
      if dim == 0 {
        p = (step, cur);
      } else {
        p = (cur, step);
      }
      let pulled = is_pulled(code, &p);
      if !prev_pulled && pulled {
        prev_start = cur;
        prev_pulled = true;
        cur = prev_stop.max(cur);
      }
      if prev_pulled && !pulled {
        prev_stop = cur;
        //println!("At step:{} - size {}", step, prev_stop-prev_start-1);
        if prev_stop-prev_start-1 >= stop_gap {
          return step;
        }
        break;
      }
      cur = cur + 1;
    }
  }
  panic!("Should never manage to exit through here");
}

fn analysis_full(code: &Vec<i64>, dim: usize, start: i64, stop: i64, stop_gap : i64) -> (HashMap<i64,(i64,i64)>,i64) {
  let mut ret : HashMap<i64,(i64,i64)> = HashMap::new();
  let mut first_good = None;
  let mut prev_start = 0;
  let mut prev_stop = 0;
  for step in start..stop {
    let mut prev_pulled = false;
    let mut cur = prev_start;
    loop {
      let mut p : Point;
      if dim == 0 {
        p = (step, cur);
      } else {
        p = (cur, step);
      }
      let pulled = is_pulled(code, &p);
      if !prev_pulled && pulled {
        prev_start = cur;
        prev_pulled = true;
        cur = prev_stop.max(cur)-1;
      }
      if prev_pulled && !pulled {
        prev_stop = cur;
        if prev_stop-prev_start-1 >= stop_gap {
          if first_good.is_none() {
            first_good = Some(step);
          }
          ret.insert(step, (prev_start, prev_stop));
        }
        break;
      }
      cur = cur + 1;
    }
  }
  return (ret,first_good.unwrap());
}

fn solve(xs : HashMap<i64,(i64,i64)>, ys: HashMap<i64,(i64,i64)>, size: i64) -> (i64,i64) {
  let mut x = xs.keys().fold( 99999, |a,i| a.min(*i));
  loop {
    if xs.contains_key(&x) {
      let y = xs.get(&x).unwrap().1-size;
      let x2 = ys.get(&y).unwrap_or(&(0,0)).1-size;
      if x <= x2 {
        println!("x:{} y:{} xs:{:?}, ys:{:?}",x,y,xs.get(&x),ys.get(&y));
        return (x, y);
      }
    }
    x = x + 1;
  }
}
fn test(code : &Vec<i64>, x: i64, y: i64, size: i64) {
  println!("({},{})", x, y);
  assert!(is_pulled(code, &(x       ,y      )));
  println!("({},{})", x+size-1, y);
  assert!(is_pulled(code, &(x+size-1,y      )));
  println!("({},{})", x, y+size-1);
  assert!(is_pulled(code, &(x       ,y+size-1)));
  println!("({},{})", x+size-1, y+size-1);
  assert!(is_pulled(code, &(x+size-1,y+size-1)));
}


fn q1(filename: impl AsRef<std::path::Path>) -> usize {
  let code = parse(filename);
  let grid = scan(&code,50,50);
  grid.print();
  return grid.data.values().filter(|c| **c == '#').count();
}

fn q2(filename: impl AsRef<std::path::Path>) -> i64 {
  let code = parse(filename);
  /*let x = analysis(&code,0,50,100);
  let y = analysis(&code,1,50,100);*/
  let xs = analysis_full(&code,0,50,2000,100);
  let ys = analysis_full(&code,1,50,2000,100);
  let (x,y) = solve(xs.0,ys.0,100);
  /*println!("{:?}", xs.0);
  println!("{:?}", ys.0);*/
  
  println!("{} {}", xs.1, ys.1);
  println!("{} {}", x, y);
  test(&code, x,y,100);
  return x*10000+y;
}
// too low 6270339
// too low 4280527

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2: {}", q2("data.txt"));
}
