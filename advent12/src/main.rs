extern crate regex;
extern crate num;

use std::fs;
use std::fmt;
use regex::Regex;
use num::integer::lcm;

type Point = [i32; 3];

struct Moon {
  position: Point,
  velocity: Point
}

impl fmt::Display for Moon {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "pos=<x={} ,y={}, z={}>, vel=<x={} ,y={}, z={}>", 
        self.position[0], self.position[1], self.position[2],
        self.velocity[0], self.velocity[1], self.velocity[2])
  }
}

struct System {
  moons: Vec<Moon>
}

impl fmt::Display for System {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for moon in &self.moons {
      writeln!(f, "{}", moon).unwrap();
    }
    return fmt::Result::Ok(());
  }
}

fn diff_clamp(a:i32,b:i32) -> i32 {
  let diff = a-b;
  if diff >= 1 { return 1; }
  if diff <= -1 { return -1; }
  return 0;
}

fn pos_diff_clamp(a:&Point, b:&Point) -> Point {
  return [
    diff_clamp(a[0],b[0]),
    diff_clamp(a[1],b[1]),
    diff_clamp(a[2],b[2])
  ];
}

impl Moon {
  fn new_from_line(line: &String) -> Moon {
    let re = Regex::new(r"^<x=([^,]+), y=([^,]+), z=([^,]+)>$").unwrap();
    let caps = re.captures(line).unwrap();
    return Moon {
      position: [caps[1].parse::<i32>().unwrap(),caps[2].parse::<i32>().unwrap(),caps[3].parse::<i32>().unwrap()],
      velocity: [0,0,0]
    }
  }
  fn compute_kinetic_energy(&self) -> i32 {
    return self.velocity.iter().fold(0, |a,v| a + v.abs());
  }
  fn compute_potential_energy(&self) -> i32 {
    return self.position.iter().fold(0, |a,p| a + p.abs());
  }
  fn compute_energy(&self) -> i32 {
    return self.compute_potential_energy() * self.compute_kinetic_energy();
  }
  fn apply_gravity(&mut self, other : &mut Moon){
    let diff = pos_diff_clamp(&self.position, &other.position);
    self.velocity[0] = self.velocity[0] - diff[0];
    self.velocity[1] = self.velocity[1] - diff[1];
    self.velocity[2] = self.velocity[2] - diff[2];
    other.velocity[0] = other.velocity[0] + diff[0];
    other.velocity[1] = other.velocity[1] + diff[1];
    other.velocity[2] = other.velocity[2] + diff[2];
  }
  fn update(&mut self){
    self.position[0] = self.position[0] + self.velocity[0];
    self.position[1] = self.position[1] + self.velocity[1];
    self.position[2] = self.position[2] + self.velocity[2];
  }
}

impl System {
  fn new_from_file(filename: impl AsRef<std::path::Path>) -> System {
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return System {
      moons: data.lines().map(|s| Moon::new_from_line(&s.to_string())).collect()
    }
  }
  fn update(&mut self) {
    let n = self.moons.len();
    for i in 0..n {
      let (head, tail) = self.moons.split_at_mut(i+1);
      for j in (i+1)..n {
        head[i].apply_gravity(&mut tail[j-(i+1)]);
      }
    }
    for moon in &mut self.moons {
      moon.update();
    }
  }
  fn compute_energy(&self) -> i32 {
    return self.moons.iter().fold(0, |a,m| a + m.compute_energy());
  }
  fn compute_kinetic_energy(&self) -> i32 {
    return self.moons.iter().fold(0, |a,m| a + m.compute_kinetic_energy());
  }
  fn hash(&self,idx: usize) -> String {
    let p : Vec<String> = self.moons.iter().map(|m| m.position[idx].to_string()).collect();
    let v : Vec<String> = self.moons.iter().map(|m| m.velocity[idx].to_string()).collect();
    return format!("{}_{}",p.join(","),v.join(","));
  }
}

fn q1(filename: impl AsRef<std::path::Path>, nb_step : usize) -> i32{
  let mut system = System::new_from_file(filename);
  //println!("{}", system);
  for step in 0..nb_step {
    system.update();
    //println!("step:{} energy:{}", step+1, system.compute_energy());
    //println!("{}", system);
  }
  return system.compute_energy();
}



fn q2(filename: impl AsRef<std::path::Path>) -> i64{
  let mut system = System::new_from_file(filename);
  let hash_x = system.hash(0);
  let hash_y = system.hash(1);
  let hash_z = system.hash(2);
  let mut step_x = 0;
  let mut step_y = 0;
  let mut step_z = 0;
  let mut step = 0;
  while step_x == 0 || step_y == 0 || step_z == 0 {
    system.update();
    //println!("step:{} energy:{}", step+1, system.compute_energy());
    //println!("{}", system);
    /*
    if system.compute_kinetic_energy() == 0 {
      println!("{}", system);
    }*/
    step = step + 1;
    if step_x == 0 && hash_x == system.hash(0) {
      println!("match x ! {}", step);
      step_x = step;
    }
    if step_y == 0 && hash_y == system.hash(1) {
      println!("match y ! {}", step);
      step_y = step;
    }
    if step_z == 0 && hash_z == system.hash(2) {
      println!("match z ! {}", step);
      step_z = step;
    }
    if step % 1000000 == 0 {
      println!("step: {}", step);
    }
  }
  return lcm(lcm(step_x,step_y),step_z);
}



fn main() {
    println!("Question1: {}", q1("data.txt",1000));
    println!("Question2: {}", q2("data.txt"));
}

#[test]
fn test_q1_examples1() {
  assert_eq!(q1("test1.txt",10), 179);
}

#[test]
fn test_q1_examples2() {
  assert_eq!(q1("test2.txt",100), 1940);
}

#[test]
fn test_q2_examples1() {
  assert_eq!(q2("test1.txt"), 2772);
}

#[test]
fn test_q2_examples2() {
  assert_eq!(q2("test2.txt"), 4686774924);
}
