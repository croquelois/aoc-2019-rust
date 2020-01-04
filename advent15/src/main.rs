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

fn follow_dir(pos: &Point, dir: i64) -> Point {
  let mut npos = *pos;
  match dir {
    1 => npos.1 = npos.1 - 1, // North
    2 => npos.1 = npos.1 + 1, // South
    3 => npos.0 = npos.0 - 1, // West
    4 => npos.0 = npos.0 + 1, // East
    _ => panic!("unexpected direction: {}", dir)
  };
  return npos;
}

fn found_return_path(pos : &Point, pathdir : &HashMap<Point,i64>) -> VecDeque<i64> {
  let mut path : VecDeque<i64> = VecDeque::new();  
  let mut cur = *pos;
  loop {
    let dir = *pathdir.get(&cur).expect("should have a direction");
    if dir == 0 { 
      return path; 
    };
    let invdir = match dir {
      1 => 2,
      2 => 1,
      3 => 4,
      4 => 3,
      _ => panic!("unexpected direction: {}", dir)
    };
    cur = follow_dir(&cur, invdir);
    path.push_back(invdir);
  }
}

fn found_path(pos : &Point, goal : &Point, pathdir : &HashMap<Point,i64>) -> VecDeque<i64> {
  let mut going_path : VecDeque<i64> = VecDeque::new();
  let mut cur = *pos;
  if pathdir.contains_key(&(goal.0,goal.1+1)) {
    cur = (goal.0,goal.1+1);
    going_path.push_front(1);
  } else if pathdir.contains_key(&(goal.0,goal.1-1)) {
    cur = (goal.0,goal.1-1);
    going_path.push_front(2);
  } else if pathdir.contains_key(&(goal.0+1,goal.1)) {
    cur = (goal.0+1,goal.1);
    going_path.push_front(3);
  } else if pathdir.contains_key(&(goal.0-1,goal.1)) {
    cur = (goal.0-1,goal.1);
    going_path.push_front(4);
  }
  loop {
    let dir = *pathdir.get(&cur).expect("should have a direction");
    if dir == 0 { break };
    let invdir = match dir {
      1 => 2,
      2 => 1,
      3 => 4,
      4 => 3,
      _ => panic!("unexpected direction: {}", dir)
    };
    cur = follow_dir(&cur, invdir);
    going_path.push_front(dir);
    //println!("fwd: {}", dir);
  }
  let mut path = found_return_path(pos, pathdir);
  while going_path.len() > 0 {
    path.push_back(going_path.pop_front().expect("checked above"));
  }
  return path;
}

fn droid(state : &mut State, explore_fully : bool) -> (Grid, usize, Point) {
  let neighbor = [(0,1),(1,0),(0,-1),(-1,0)];
  let mut grid = Grid::new();
  let mut path : VecDeque<i64> = VecDeque::new();
  let mut pathdir : HashMap<Point,i64> = HashMap::new();
  let mut p : Point = (0,0);
  let mut oxygen : Point = (0,0);
  let mut open : VecDeque<Point> = VecDeque::new();
  open.push_back((0,1));
  open.push_back((1,0));
  open.push_back((0,-1));
  grid.insert((0,0), '.');
  pathdir.insert((0,0), 0);
  path.push_back(1);
  while !state.finished {
    let mut input : VecDeque<i64> = VecDeque::new();
    let dir = path.pop_front().expect("should not be empty");
    input.push_back(dir);
    let np = follow_dir(&p, dir);
    //println!("p: {:?} np: {:?}", p, np);
    //println!("input: {:?}", input);
    let mut output = state.process(&mut input);
    //println!("output: {:?}", output);
    let status = output.pop_front().expect("should have a status output");
    match status {
      0 => { // hit a wall
        grid.insert(np, '#');
        //path = search_new_path();
      },
      1 => { // move ok
        grid.insert(np, '.');
        p = np;
      },
      2 => { // move ok, oxygen
        grid.insert(np, 'o');
        p = np;
        oxygen = p;
        if !explore_fully {
          pathdir.insert(p, dir);
          return (grid,found_return_path(&p, &pathdir).len(),oxygen);
        }
      },
      _ => panic!("unexpected status")
    };
    if path.len() == 0 {
      if status != 0 {
        pathdir.insert(p, dir);
        for n in &neighbor { 
          let nbp = (p.0+n.0,p.1+n.1);
          if !grid.contains_key(&nbp) { 
            open.push_back(nbp); 
          }
        }
      }
      if open.len() == 0 && explore_fully {
        return (grid,0,oxygen);
      }
      let goal = open.pop_front().expect("should not be out of option before oxygen tank found");
      //println!("New goal: {:?}", goal);
      path = found_path(&p, &goal, &pathdir);
      //println!("Path: {:?}", path);
      /*println!("*********************");
      print_grid_minmax(&grid, -20, 20, -20, 20);*/
    }
    
    /*println!("*********************");
    print_grid_minmax(&grid, -10, 10, -10, 10);*/
    /*println!("count {} move {} color {}", grid.len(), movement, color);
    */
    //print_grid(&grid);
  }
  panic!("should not exit");
}

fn expand_oxygen(from : &Point, grid : &Grid) -> usize {
  let neighbor = [(0,1),(1,0),(0,-1),(-1,0)];
  let mut open : VecDeque<Point> = VecDeque::new();
  let mut close : HashMap<Point, usize> = HashMap::new();
  let mut max = 0;
  close.insert(*from, 0);
  open.push_back(*from);
  while open.len() > 0 {
    let p = open.pop_front().expect("checked above");
    let cost = *close.get(&p).unwrap();
    for n in &neighbor { 
      let nbp = (p.0+n.0,p.1+n.1);
      if !close.contains_key(&nbp) && *grid.get(&nbp).expect("map is bounded") == '.' { 
        close.insert(nbp, cost+1);
        open.push_back(nbp);
        max = cost+1;
      }
    }
  }
  return max;
}

fn q1(filename: impl AsRef<std::path::Path>) -> usize {
  let mut state = State::new_from_file(filename);
  let (grid, length, _) = droid(&mut state, false);
  print_grid(&grid);
  return length;
}

fn q2(filename: impl AsRef<std::path::Path>) -> usize {
  let mut state = State::new_from_file(filename);
  let (grid, _, oxygen) = droid(&mut state, true);
  print_grid(&grid);
  return expand_oxygen(&oxygen, &grid);
}

/*
fn q2(filename: impl AsRef<std::path::Path>) -> i64 {
  let mut state = State::new_from_file(filename);
  state.set_mem(0,2);
  return game(&mut state).1;
}
*/
fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2: {}", q2("data.txt"));
  //println!("Question2: {}", q2("data.txt"));
}
