#![allow(dead_code)]
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

type Point = (i32,i32);
type Point3D = (i32,i32,i32);
const NEIGHBOR : [Point;4] = [(0,1),(1,0),(0,-1),(-1,0)];

struct Map {
  floor: HashSet<Point>,
  teleporters: HashMap<Point,(Point,bool)>,
  start: Point,
  end: Point
}

fn get_one_freepos_around(floor: &HashSet<Point>, pos: &Point) -> Option<Point> {
  for nb in &NEIGHBOR {
    let pt = (pos.0+nb.0,pos.1+nb.1);
    if floor.contains(&pt) {
      return Some(pt);
    }
  }
  return None;
}

fn minmax(floor : &HashSet<Point>) -> (i32, i32, i32, i32) {
  let min_x = floor.iter().fold( 99999, |a,i| a.min(i.0));
  let max_x = floor.iter().fold(-99999, |a,i| a.max(i.0));
  let min_y = floor.iter().fold( 99999, |a,i| a.min(i.1));
  let max_y = floor.iter().fold(-99999, |a,i| a.max(i.1));
  return (min_x, max_x, min_y, max_y);
}

fn is_on_border(border: &(i32, i32, i32, i32), pt: &Point) -> bool{
  return pt.0 == border.0 || pt.0 == border.1 || pt.1 == border.2 || pt.1 == border.3;
}
  
impl Map {  
  fn read_from_file(filename: impl AsRef<std::path::Path>) -> Self {
    let mut floor : HashSet<Point> = HashSet::new();
    let mut chrs : HashMap<Point,char> = HashMap::new();
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut y = 0;
    for line in data.lines() {
      let mut x = 0;
      for chr in line.chars() {
        if chr == '.' { floor.insert((x, y)); }
        else if chr != '#' && chr != ' ' { chrs.insert((x, y), chr); }
        x = x + 1;
      }
      y = y + 1;
    }
    let mut opt_start = None;
    let mut opt_end = None;
    let mut teleporters_name : HashMap<String,Vec<Point>> = HashMap::new();
    for (key,c1) in &chrs {
      let mut opt_c2 = chrs.get(&(key.0+1,key.1));
      if opt_c2.is_none() { opt_c2 = chrs.get(&(key.0,key.1+1)); };
      if !opt_c2.is_none() {
        let c2 = opt_c2.unwrap();
        let name : String = format!("{}{}", c1, c2);
        let mut opt_pos = get_one_freepos_around(&floor, &key);
        if opt_pos.is_none() { opt_pos = get_one_freepos_around(&floor, &(key.0+1,key.1)); }
        if opt_pos.is_none() { opt_pos = get_one_freepos_around(&floor, &(key.0,key.1+1)); }
        let pos = opt_pos.expect("no free position around");
        match name.as_str() {
          "AA" => opt_start = Some(pos),
          "ZZ" => opt_end = Some(pos),
          _ => {
            if !teleporters_name.contains_key(&name) {
              teleporters_name.insert(name.to_string(), Vec::new());
            }
            teleporters_name.get_mut(&name).unwrap().push(pos);
          }
        } 
      }
    }
    let border = minmax(&floor);
    let mut teleporters : HashMap<Point,(Point,bool)> = HashMap::new();
    for (_,pts) in teleporters_name {
      assert_eq!(pts.len(),2);
      teleporters.insert(pts[0],(pts[1],is_on_border(&border, &pts[0])));
      teleporters.insert(pts[1],(pts[0],is_on_border(&border, &pts[1])));
    }
    return Map {
      floor: floor,
      teleporters: teleporters,
      start: opt_start.unwrap(),
      end: opt_end.unwrap()
    };
  }
  
  fn get_neighbor_rec(&self, pos: &Point, lvl: usize) -> impl Iterator<Item = (Point,usize)> {
    let mut ret : Vec<(Point,usize)> = Vec::new();
    let teleport = self.teleporters.get(pos);
    if !teleport.is_none() {
      let (p, outer) = *teleport.unwrap();
      if outer {
        if lvl > 0 {
          ret.push((p,(lvl-1) as usize));
        }
      } else {
        ret.push((p,(lvl+1) as usize));
      }
      
    }
    for nb in &NEIGHBOR {
      let p = (pos.0+nb.0,pos.1+nb.1);
      if self.is_free(&p) {
        ret.push((p,lvl));
      }
    }
    return ret.into_iter();
  }
  
  fn get_neighbor(&self, pos: &Point) -> impl Iterator<Item = Point> {
    let mut ret : Vec<Point> = Vec::new();
    let teleport = self.teleporters.get(pos);
    if !teleport.is_none() {
      ret.push(teleport.unwrap().0);
    }
    for nb in &NEIGHBOR {
      let p = (pos.0+nb.0,pos.1+nb.1);
      if self.is_free(&p) {
        ret.push(p);
      }
    }
    return ret.into_iter();
  }

  fn is_free(&self, pos : &Point) -> bool {
    return self.floor.contains(pos);
  }

  fn print_minmax(&self, min_x: i32, max_x: i32, min_y:i32, max_y:i32) {
    for y in min_y..(max_y+1) {
      let mut line = Vec::new();
      for x in min_x..(max_x+1) {
        line.push(match self.is_free(&(x,y)){ true => '.', false => '#'});
      }
      let line_str : String = line.into_iter().collect();
      println!("{}", line_str);
    }
  }

  fn minmax(&self) -> (i32, i32, i32, i32) {
    return minmax(&self.floor);
  }
  
  fn print(&self) {
    let (min_x, max_x, min_y, max_y) = self.minmax();
    self.print_minmax(min_x, max_x, min_y, max_y);
  }
}

fn search(map : &Map) -> Option<usize> {
  let mut open : VecDeque<Point> = VecDeque::new();
  let mut close : HashMap<Point,usize> = HashMap::new();
  open.push_back(map.start);
  close.insert(map.start,0);
  while !open.is_empty() {
    let cur = open.pop_front().expect("checked above");
    let cost = *close.get(&cur).expect("everything in open should be in close") + 1;
    for nb in map.get_neighbor(&cur) {
      if nb == map.end {
        return Some(cost);
      }
      if !close.contains_key(&nb) {
        open.push_back(nb);
        close.insert(nb, cost);
      }
    }
  }
  return None;
}

fn search_rec(map : &Map) -> Option<usize> {
  let mut open : VecDeque<(Point,usize)> = VecDeque::new();
  let mut close : HashMap<(Point,usize),usize> = HashMap::new();
  open.push_back((map.start,0));
  close.insert((map.start,0),0);
  while !open.is_empty() {
    let (cur,lvl) = open.pop_front().expect("checked above");
    let cost = *close.get(&(cur,lvl)).expect("everything in open should be in close") + 1;
    for nb in map.get_neighbor_rec(&cur,lvl) {
      if nb.0 == map.end && nb.1 == 0 {
        return Some(cost);
      }
      if !close.contains_key(&nb) {
        open.push_back(nb);
        close.insert(nb, cost);
      }
    }
  }
  return None;
}

fn q1(filename: impl AsRef<std::path::Path>) -> usize {
  let map = Map::read_from_file(filename);
  return search(&map).expect("it should have a solution");
}

fn q2(filename: impl AsRef<std::path::Path>) -> usize {
  let map = Map::read_from_file(filename);
  return search_rec(&map).expect("it should have a solution");
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2: {}", q2("data.txt"));
}


#[test]
fn test_q1_examples1() {
  assert_eq!(q1("test1.txt"), 23);
}

#[test]
fn test_q1_examples2() {
  assert_eq!(q1("test2.txt"), 58);
}

#[test]
fn test_q2_examples1() {
  assert_eq!(q2("test1.txt"), 26);
}

#[test]
fn test_q2_examples2() {
  assert_eq!(q2("test3.txt"), 396);
}