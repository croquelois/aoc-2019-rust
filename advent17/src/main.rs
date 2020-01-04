#![allow(dead_code)]
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

  fn print_minmax(&self, min_x: i32, max_x: i32, min_y:i32, max_y:i32) {
    for y in min_y..(max_y+1) {
      let mut line = Vec::new();
      for x in min_x..(max_x+1) {
        line.push(self.get(&(x,y)));
      }
      let line_str : String = line.into_iter().collect();
      println!("{}", line_str);
    }
  }

  fn minmax(&self) -> (i32, i32, i32, i32) {
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

fn camera(state : &mut State) -> Grid {
  let mut grid = Grid::new();
  while !state.finished {
    let mut input : VecDeque<i64> = VecDeque::new();
    let output = state.process(&mut input);
    grid = output_to_grid(&output).0;
  }
  return grid;
}

fn get_crossing(grid : &Grid) -> Vec<Point> {
  let mut ret = Vec::new();
  let neighbor = [(0,1),(1,0),(0,-1),(-1,0)];
  let (min_x, max_x, min_y, max_y) = grid.minmax();
  for y in (min_y+1)..max_y {
    for x in (min_x+1)..max_x {
      if grid.get(&(x,y)) != '.' {
        let mut nb = 0;
        for n in &neighbor {
          if grid.get(&(x+n.0,y+n.1)) != '.' {
            nb = nb + 1;
          }
        }
        if nb == 4 {
          ret.push((x,y));
        }
      }
    }
  }
  return ret;
}

fn q1(filename: impl AsRef<std::path::Path>) -> i32 {
  let mut state = State::new_from_file(filename);
  let grid = camera(&mut state);
  //grid.print();
  let crossing = get_crossing(&grid);
  return crossing.into_iter().fold(0,|a,p| a + p.0*p.1);
}

fn found_robot(grid : &Grid) -> Option<(char,Point)> {
  let (min_x, max_x, min_y, max_y) = grid.minmax();
  for y in min_y..(max_y+1) {
    for x in min_x..(max_x+1) {
      let pos = (x,y);
      let c = grid.get(&pos);
      match c {
        '<' => return Some(('W', pos)),
        '>' => return Some(('E', pos)),
        '^' => return Some(('N', pos)),
        'v' => return Some(('S', pos)),
        _ => ()
      }
    }
  }
  return None;
}

fn follow_dir(pos: &Point, dir: char) -> Point {
  let mut npos = *pos;
  match dir {
    'W' => npos.0 = npos.0 - 1, // West
    'E' => npos.0 = npos.0 + 1, // East
    'N' => npos.1 = npos.1 - 1, // North
    'S' => npos.1 = npos.1 + 1, // South
    _ => panic!("unexpected direction: {}", dir)
  };
  return npos;
}

fn follow_turn(dir: char, turn: char) -> char {
  return match dir {
    'W' => match turn { 'L' => 'S' , 'R' => 'N', _ => panic!("unexpected turn: {}", turn) }
    'E' => match turn { 'L' => 'N' , 'R' => 'S', _ => panic!("unexpected turn: {}", turn) }
    'N' => match turn { 'L' => 'W' , 'R' => 'E', _ => panic!("unexpected turn: {}", turn) }
    'S' => match turn { 'L' => 'E' , 'R' => 'W', _ => panic!("unexpected turn: {}", turn) }
    _ => panic!("unexpected direction: {}", dir)
  };
}

enum Action {
  Move(usize),
  Turn(char)
}

impl Action {
  fn to_string(&self) -> String {
    return match self {
      Action::Move(n) => n.to_string(),
      Action::Turn(c) => c.to_string()
    }
  }
}

fn compute_path(grid : &Grid) -> Vec<Action> {
  let mut path = Vec::new();
  let mut cur = found_robot(grid).expect("the robot was not found !");
  let mut count = 0;
  loop {
    let (dir, pos) = cur;
    let npos = follow_dir(&pos, dir);
    let c = grid.get(&npos);
    match c {
      '#' => {
        count = count + 1;
        cur = (dir, npos);
      },
      '.' | ' ' => {
        if count > 0 {
          path.push(Action::Move(count));
          count = 0;
        }
        if grid.get(&follow_dir(&pos, follow_turn(dir,'L'))) == '#' {
          path.push(Action::Turn('L'));
          cur = (follow_turn(dir,'L'), pos);
        } else if grid.get(&follow_dir(&pos, follow_turn(dir,'R'))) == '#' {
          path.push(Action::Turn('R'));
          cur = (follow_turn(dir,'R'), pos);
        } else {
          if count > 0 {
            path.push(Action::Move(count));
          }
          return path;
        }
      }
      _ => panic!("unexpected character uncountered: {}", c)
    }
  }
}

fn best_compression(path : Vec<Action>) -> (Vec<char>,Vec<Vec<Action>>) {
  let mut move_fcts = Vec::new();
  let mut move_routine = Vec::new();
  // TODO
  move_routine.push('A');
  move_fcts.push(path);
  return (move_routine, move_fcts);
}

fn create_string_from_move_fct(move_fct : &Vec<Action>) -> String {
  return move_fct.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(",");
}

fn create_string(move_routine : Vec<char>, move_fcts : Vec<Vec<Action>>) -> String {
  let move_routine_string = move_routine.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
  let move_fcts_string = move_fcts.iter().map(create_string_from_move_fct).collect::<Vec<String>>().join("\n");
  return format!("{}\n{}\nn", move_routine_string, move_fcts_string);
}

fn string_to_input(input_string : String) -> VecDeque<i64> {
  return input_string.chars().map(|c| c as u8 as i64).collect();
}

fn q2(filename: impl AsRef<std::path::Path>) -> i64 {
  let mut state = State::new_from_file(&filename);
  let grid = camera(&mut state);

  let path = compute_path(&grid);
  let (move_routine, move_fcts) = best_compression(path);
  let input_string = create_string(move_routine, move_fcts);
  println!("{}", input_string);
  let mut robot_input = string_to_input(input_string);

  let mut state2 = State::new_from_file(&filename);
  state2.set_mem(0, 2);
  let output = state2.process(&mut robot_input);

  return *output.iter().next().expect("should output the amount of dust");
}

fn q2_prepare(filename: impl AsRef<std::path::Path>) -> String {
  let mut state = State::new_from_file(&filename);
  let grid = camera(&mut state);
  return create_string_from_move_fct(&compute_path(&grid));
}

fn q2_send(filename: impl AsRef<std::path::Path>) -> i64 {
  let solution = 
"A,A,B,C,B,C,B,A,C,A
R,8,L,12,R,8
L,10,L,10,R,8
L,12,L,12,L,10,R,10
n
";
  let mut robot_input = string_to_input(solution.to_string());

  let mut state = State::new_from_file(&filename);
  state.set_mem(0, 2);
  let output = state.process(&mut robot_input);
  let (grid,unprocessed) = output_to_grid(&output);
  //grid.print();
  return *unprocessed.iter().next().expect("should output the amount of dust");
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2-prepare: {}", q2_prepare("data.txt"));
  println!("Question2-send: {}", q2_send("data.txt"));
}
