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

type Point = (i32,i32);
type Grid = HashMap<Point,char>;

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

fn print_grid_minmax(grid : &Grid, min_x: i32, max_x: i32, min_y:i32, max_y:i32) {
  for y in min_y..(max_y+1) {
    let mut line = Vec::new();
    for x in min_x..(max_x+1) {
      line.push(*grid.get(&(x,y)).unwrap_or(&' '));
    }
    let line_str : String = line.into_iter().collect();
    println!("{}", line_str);
  }
}

fn print_grid(grid : &Grid) {
  let min_x = grid.keys().fold(99999, |a,i| a.min(i.0));
  let min_y = grid.keys().fold(99999, |a,i| a.min(i.1));
  let max_x = grid.keys().fold(-99999, |a,i| a.max(i.0));
  let max_y = grid.keys().fold(-99999, |a,i| a.max(i.1));
  print_grid_minmax(&grid, min_x, max_x, min_y, max_y);
}

fn game(state : &mut State) -> (Grid,i64) {
  let mut grid = Grid::new();
  let mut pos_ball : Point = (0,0);
  let mut pos_pad : Point = (0,0);
  let mut score = 0;
  while !state.finished {
    let mut input : VecDeque<i64> = VecDeque::new();
    if pos_ball.0 < pos_pad.0 {
      input.push_back(-1);
    } else if pos_ball.0 > pos_pad.0 {
      input.push_back(1);
    } else {
      input.push_back(0);
    }
    let output = state.process(&mut input);
    let mut output_iter = output.iter();
    loop {
      let tmp = output_iter.next();
      if !tmp.is_some() { break }
      let x = *tmp.expect("should output X") as i32;
      let y = *output_iter.next().expect("should output Y") as i32;
      let t = output_iter.next().expect("should output Type");
      if x == -1 && y == 0 {
        score = *t;
      } else {
        let c = match t {
          0 => ' ',
          1 => '#',
          2 => 'H',
          3 => '_',
          4 => 'o',
          _ => panic!("unexpected type")
        };
        if c == 'o' {
          pos_ball = (x,y);
        }
        if c == '_' {
          pos_pad = (x,y);
        }
        grid.insert((x,y), c);
      }
      /*
      println!("*********************");
      print_grid_minmax(&grid, -10, 10, -10, 10);
      println!("count {} move {} color {}", grid.len(), movement, color);
      */
    }
    //print_grid(&grid);
  }
  return (grid,score);
}

fn q1(filename: impl AsRef<std::path::Path>) -> usize {
  let mut state = State::new_from_file(filename);
  let grid = game(&mut state).0;
  return grid.values().filter(|c| **c == 'H').count();
}

fn q2(filename: impl AsRef<std::path::Path>) -> i64 {
  let mut state = State::new_from_file(filename);
  state.set_mem(0,2);
  return game(&mut state).1;
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2: {}", q2("data.txt"));
}
