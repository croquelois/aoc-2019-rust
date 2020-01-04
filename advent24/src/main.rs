#![allow(dead_code,unused_imports)]
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

type Point = (i32,i32);
const NEIGHBOR : [Point;4] = [(0,1),(1,0),(0,-1),(-1,0)];

struct Planet {
  bugs: Vec<Vec<bool>>,
}

impl Planet {
  fn from_file(filename: impl AsRef<std::path::Path>) -> Self {
    let mut bugs = vec![vec![false; 5]; 5];
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut y = 0;
    for line in data.lines() {
      let mut x = 0;
      for chr in line.chars() {
        if chr == '#' {
          bugs[x][y] = true;
        }
        x = x + 1;
      }
      y = y + 1;
    }
    return Planet {
      bugs: bugs
    };
  }
  fn has_bugs(&self, pos: &Point) -> bool {
    let (x,y) = *pos;
    if x < 0 || x >= 5 || y < 0 || y >= 5 { return false; }
    return self.bugs[x as usize][y as usize];
  }
  fn count_around(&self, pos: &Point) -> usize {
    let mut n = 0;
    for nb in &NEIGHBOR {
      if self.has_bugs(&(pos.0+nb.0,pos.1+nb.1)) {
        n = n + 1;
      }
    }
    return n;
  }
  fn evolve(&mut self) {
    let mut bugs = vec![vec![false; 5]; 5];
    for x in 0..5 {
      for y in 0..5 {
        let p = (x,y);
        let n = self.count_around(&p);
        if self.has_bugs(&p) {
          if n == 1 {
            bugs[x as usize][y as usize] = true;
          }
        } else {
          if n == 1 || n == 2 {
            bugs[x as usize][y as usize] = true;
          }
        }
      }
    }
    self.bugs = bugs;
  }
  fn print(&self) {
    for y in 0..5 {
      let mut line = Vec::new();
      for x in 0..5 {
        line.push(if self.has_bugs(&(x,y)) { '#' } else { '.' });
      }
      let line_str : String = line.into_iter().collect();
      println!("{}", line_str);
    }
  }
  fn hash(&self) -> u64 {
    let mut hash : u64 = 0;
    let mut p : u64 = 1;
    for y in 0..5 {
      for x in 0..5 {
        if self.has_bugs(&(x,y)) { 
          hash = hash + p;
        }
        p = p * 2;
      }
    }
    return hash;
  }
}

struct PlanetRec {
  bugs: HashMap<i32,Vec<Vec<bool>>>,
}

impl PlanetRec {
  fn from_file(filename: impl AsRef<std::path::Path>) -> Self {
    let mut bugs = vec![vec![false; 5]; 5];
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut y = 0;
    for line in data.lines() {
      let mut x = 0;
      for chr in line.chars() {
        if chr == '#' {
          bugs[x][y] = true;
        }
        x = x + 1;
      }
      y = y + 1;
    }
    let mut ret : HashMap<i32,Vec<Vec<bool>>> = HashMap::new();
    ret.insert(0, bugs);
    return PlanetRec {
      bugs: ret
    };
  }
  fn has_bugs(&self, pos: &Point, lvl: i32) -> bool {
    let (x,y) = *pos;
    if x < 0 || x >= 5 || y < 0 || y >= 5 { return false; }
    let opt = self.bugs.get(&lvl);
    if opt.is_none() { return false; }
    return opt.unwrap()[x as usize][y as usize];
  }
  fn get_neighbor_helper(&self, from: &Point, to: &Point, lvl: i32) -> Vec<(Point,i32)> {
	let (x,y) = *to;
	if x == -1 {
		return vec![((1,2) as Point,lvl-1)];
	}
	if x == 5 {
		return vec![((3,2) as Point,lvl-1)];
	}
	if y == -1 {
		return vec![((2,1) as Point,lvl-1)];
	}
	if y == 5 {
		return vec![((2,3) as Point,lvl-1)];
	}
	if x == 2 && y == 2 {
		if from.0 < 2 {
			return vec![((0,0) as Point,lvl+1),((0,1) as Point,lvl+1),((0,2) as Point,lvl+1),((0,3) as Point,lvl+1),((0,4) as Point,lvl+1)];
		}
		if from.0 > 2 {
			return vec![((4,0) as Point,lvl+1),((4,1) as Point,lvl+1),((4,2) as Point,lvl+1),((4,3) as Point,lvl+1),((4,4) as Point,lvl+1)];
		}
		if from.1 < 2 {
			return vec![((0,0) as Point,lvl+1),((1,0) as Point,lvl+1),((2,0) as Point,lvl+1),((3,0) as Point,lvl+1),((4,0) as Point,lvl+1)];
		}
		if from.1 > 2 {
			return vec![((0,4) as Point,lvl+1),((1,4) as Point,lvl+1),((2,4) as Point,lvl+1),((3,4) as Point,lvl+1),((4,4) as Point,lvl+1)];
		}
	}
	return vec![(*to,lvl)];
  }
  fn get_neighbor(&self, pos: &Point, lvl: i32) -> impl Iterator<Item = (Point,i32)> {
	let (x,y) = *pos;
    assert!(!(x < 0 || x >= 5 || y < 0 || y >= 5));
    assert!(!(x == 2 && y == 2));
	let mut vec : Vec<(Point,i32)> = Vec::new();
	for nb in &NEIGHBOR {
		vec.extend(self.get_neighbor_helper(&pos,&(x+nb.0,y+nb.1),lvl));
	}
	return vec.into_iter();
  }
  fn count_around(&self, pos: &Point, lvl: i32) -> usize {
    let mut n = 0;
    for (p,lvl) in self.get_neighbor(pos, lvl) {
      if self.has_bugs(&p, lvl) {
        n = n + 1;
      }
    }
    return n;
  }
  fn evolve(&mut self) {
    let mut whole_map : HashMap<i32,Vec<Vec<bool>>> = HashMap::new(); //
    let min_lvl = self.bugs.keys().fold( 99999, |a,i| a.min(*i));
    let max_lvl = self.bugs.keys().fold( -99999, |a,i| a.max(*i));
    
    for lvl in (min_lvl-1)..(max_lvl+2) {
      let mut nb = 0;
      let mut bugs = vec![vec![false; 5]; 5];
      for x in 0..5 {
        for y in 0..5 {
          if x == 2 && y == 2 { continue; }
          let p = (x,y);
          let n = self.count_around(&p, lvl);
		  //println!("count around {:?} : {} has? {}", p, n, self.has_bugs(&p, lvl));
          if self.has_bugs(&p, lvl) {
            if n == 1 {
              bugs[x as usize][y as usize] = true;
              nb = nb + 1;
            }
          } else {
            if n == 1 || n == 2 {
              bugs[x as usize][y as usize] = true;
              nb = nb + 1;
            }
          }
        }
      }
      if nb > 0 {
        whole_map.insert(lvl, bugs);
      } else {
		//println!("eliminate {}", lvl);
	  }
    }
    self.bugs = whole_map;
  }
  fn count_bugs(&self) -> usize {
    let mut nb = 0;
    for key in self.bugs.keys() {
      for x in 0..5 {
        for y in 0..5 {
          let p = (x,y);
          if self.has_bugs(&p, *key) {
            nb = nb + 1;
          }
        }
      }
    }
    return nb;
  }
  fn print(&self, lvl: i32) {
    for y in 0..5 {
      let mut line = Vec::new();
      for x in 0..5 {
        line.push(if self.has_bugs(&(x,y), lvl) { '#' } else { '.' });
      }
      let line_str : String = line.into_iter().collect();
      println!("{}", line_str);
    }
  }
}

fn q1(filename: impl AsRef<std::path::Path>) -> u64 {
  let mut already_seen : HashSet<u64> = HashSet::new();
  let mut planet = Planet::from_file(filename);
  loop {
    planet.evolve();
    let hash = planet.hash();
    if !already_seen.insert(hash) {
      return hash;
    }
  }
}

fn q2(filename: impl AsRef<std::path::Path>, nb_min : i32) -> usize {
  let mut planet = PlanetRec::from_file(filename);
  //planet.print(0);
  for min in 0..nb_min {
    planet.evolve();
	//planet.print(0);
  }
  return planet.count_bugs();
}

fn main(){
  println!("Question 1: {}", q1("data.txt"));
  println!("Question 2: {}", q2("data.txt", 200));
}

#[test]
fn test_q1_examples1() {
  assert_eq!(q1("test.txt"), 2129920);
}

#[test]
fn test_q2_examples1() {
  assert_eq!(q2("test.txt", 10), 99);
}