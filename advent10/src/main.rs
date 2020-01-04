extern crate num;

use std::fs;
use std::collections::HashSet;
use num::integer::gcd;
//use std::collections::HashMap;
//use std::collections::VecDeque;

type Point = (i32,i32);
type Field = HashSet<Point>;


fn parse(filename: impl AsRef<std::path::Path>) -> Field {
  let mut field = Field::new();
  let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let mut y = 0;
  for line in data.lines() {
    let mut x = 0;
    for chr in line.chars() {
      if chr == '#' { field.insert((x, y)); }
      x = x + 1;
    }
    y = y + 1;
  }
  return field;
}

fn is_empty(field: &Field, p: &Point) -> bool {
  return !field.contains(p);
}

fn has_los(field: &Field, from: &Point, to: &Point) -> bool {
  let diff = (to.0 - from.0, to.1 - from.1);
  let n = gcd(diff.0,diff.1);
  //println!("from:{:?} to:{:?} n:{:?} diff:{:?}", from, to, n, diff);
  for i in 1..n {
    let p = (from.0 + i * diff.0 / n, from.1 + i * diff.1 / n);
    //println!("{:?}", p);
    if !is_empty(field, &p) { return false; }
  }
  return true;
}

fn get_best_pos(field : &Field) -> (i32, Point) {
  let mut best_count = 0;
  let mut best_pos = (0,0);
  for from in field {
    let mut count = 0;
    for to in field {
      if has_los(&field, &from, &to) { count = count + 1; }
    }
    if best_count < count {
      best_count = count;
      best_pos = *from;
    }
  }
  return (best_count - 1, best_pos);
}

fn get_los_asteroids(field: &Field, source: &Point) -> Field {
  //println!("collecting");
  let mut asteroids = Field::new();
  for to in field {
    if has_los(field, source, &to) { 
      asteroids.insert(*to); 
    }
  }
  return asteroids;
}

fn compute_angle(from: &Point, to: &Point) -> f64 {
  //println!("compute {:?} {:?}", from, to);
  let diff = ((to.0 - from.0) as f64, (to.1 - from.1) as f64);
  let rayon = (diff.0*diff.0 + diff.1*diff.1).sqrt();
  let angle = (diff.1/rayon).asin();
  //println!("angle {}", angle);
  if diff.0 < 0.0 {
    return std::f64::consts::PI - angle
  }
  return angle;
}

fn order_by_angle(asteroids: &Field, source: &Point) -> Vec<Point> {
  //println!("ordering");
  let mut ret : Vec<(&Point, f64)> = asteroids.iter().map(|i| (i,compute_angle(source, i))).collect();
  //println!("the vector: {:?}", ret);
  ret.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  //println!("done ordering");
  return ret.iter().map(|t| *t.0).collect();
}

fn remove_asteroids(field: &mut Field, asteroids: &Field) {
  //println!("removing");
  for ast in asteroids { field.remove(ast); }
}

fn q1(filename: impl AsRef<std::path::Path>) -> i32 {
  let field = parse(filename);
  return get_best_pos(&field).0;
}

fn q2(filename: impl AsRef<std::path::Path>, index: i32) -> i32 {
  let mut field = parse(filename);
  let station = get_best_pos(&field).1;
  field.remove(&station);
  let mut total_destroyed : i32 = 0;
  while field.len() > 0 {
    let asteroids = get_los_asteroids(&field, &station);
    if total_destroyed + (asteroids.len() as i32) > (index-1) {
      let ordered = order_by_angle(&asteroids, &station);
      let pt = ordered[(index - 1 - total_destroyed) as usize];
      return pt.0 * 100 + pt.1;
    }
    remove_asteroids(&mut field, &asteroids);
    total_destroyed = total_destroyed + (asteroids.len() as i32);
  }
  panic!("not expected to finish before 200 asteroids destroyed");
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2: {}", q2("data.txt",200));
}

#[test]
fn test_q1_examples1() {
  assert_eq!(q1("test1.txt"), 33);
}

#[test]
fn test_q1_examples2() {
  assert_eq!(q1("test2.txt"), 35);
}

#[test]
fn test_q1_examples3() {
  assert_eq!(q1("test3.txt"), 41);
}

#[test]
fn test_q1_examples4() {
  assert_eq!(q1("test4.txt"), 210);
}

#[test]
fn test_q2_examples1() {
  assert_eq!(q2("test4.txt", 1), 1112);
}

#[test]
fn test_q2_examples2() {
  assert_eq!(q2("test4.txt", 2), 1201);
}

#[test]
  fn test_q2_examples3() {
  assert_eq!(q2("test4.txt", 3), 1202);
}

#[test]
fn test_q2_examples10() {
  assert_eq!(q2("test4.txt", 10), 1208);
}

#[test]
fn test_q2_examples20() {
  assert_eq!(q2("test4.txt", 20), 1600);
}

#[test]
fn test_q2_examples50() {
  assert_eq!(q2("test4.txt", 50), 1609);
}

#[test]
fn test_q2_examples100() {
  assert_eq!(q2("test4.txt", 100), 1016);
}

#[test]
fn test_q2_examples199() {
  assert_eq!(q2("test4.txt", 199), 906);
}

#[test]
fn test_q2_examples200() {
  assert_eq!(q2("test4.txt", 200), 802);
}

#[test]
fn test_q2_examples201() {
  assert_eq!(q2("test4.txt", 201), 1009);
}

#[test]
fn test_q2_examples299() {
  assert_eq!(q2("test4.txt", 299), 1101);
}