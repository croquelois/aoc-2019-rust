#![allow(dead_code,unused_imports)]

use std::fs;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::cmp::Ordering;


type Point = (i32,i32);
const NEIGHBOR : [Point;4] = [(0,1),(1,0),(0,-1),(-1,0)];

//// GRID ////

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

  fn new_from_string(string : &String) -> Grid {
    let mut grid = Grid::new();
    let mut p : Point = (0,0);
    for o in string.chars() {
      match o {
        '\n' => {
          p.0 = 0;
          p.1 = p.1 + 1;
        },
        _ => {
          grid.set(p, o);
          p.0 = p.0 + 1;
        }
      }
    }
    return grid;
  }

  fn new_from_file(filename: impl AsRef<std::path::Path>) -> Grid {
    return Grid::new_from_string(&fs::read_to_string(filename).expect("Something went wrong reading the file"));
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

//// (END) GRID (END) ////

//// PATH AND COST ////

#[derive(Eq,Clone)]
struct PathAndCost {
  path : Vec<char>,
  cost : usize
}

impl PartialEq for PathAndCost {
  fn eq(&self, other: &Self) -> bool { self.cost == other.cost }
}
impl Ord for PathAndCost {
  fn cmp(&self, other: &Self) -> Ordering { other.cost.cmp(&self.cost) }
}
impl PartialOrd for PathAndCost {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl PathAndCost {
  fn get_keys(&self) -> HashSet<char> {
    HashSet::from_iter(self.path.iter().filter(|c| c.is_ascii_lowercase()).map(|c| *c))
  }
  fn get_doors(&self) -> HashSet<char> {
    HashSet::from_iter(self.path.iter().filter(|c| c.is_ascii_uppercase()).map(|c| *c))
  }
  fn get_open_doors(&self) -> HashSet<char> {
    HashSet::from_iter(self.path.iter().filter(|c| c.is_ascii_lowercase()).map(|c| c.to_ascii_uppercase()))
  }
  fn get_nodes(&self) -> HashSet<char> {
    HashSet::from_iter(self.path.iter().map(|c| *c))
  }
  fn get_last(&self) -> char {
    self.path[self.path.len()-1]
  }
  fn hash(&self) -> String {
    return self.path.iter().collect();
  }
  fn add_node(&self, node: char, cost: usize) -> Self {
    let mut new_path = self.path.to_vec();
    new_path.push(node);
    return Self {
      path: new_path,
      cost: self.cost + cost
    }
  }
}

//// (END) PATH AND COST (END) ////

//// PATH AND COST MULTIBOT ////

#[derive(Eq,Clone)]
struct PathAndCostMultiBot {
  path: Vec<char>,
  last: Vec<char>,
  cost: usize,
}

impl PartialEq for PathAndCostMultiBot {
  fn eq(&self, other: &Self) -> bool { self.cost == other.cost }
}
impl Ord for PathAndCostMultiBot {
  fn cmp(&self, other: &Self) -> Ordering { other.cost.cmp(&self.cost) }
}
impl PartialOrd for PathAndCostMultiBot {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl PathAndCostMultiBot {
  fn initial() -> Self {
    return Self {
      path: Vec::new(),
      last: vec!['@','%','$','&'],
      cost: 0
    }
  }
  fn get_keys(&self) -> HashSet<char> {
    HashSet::from_iter(self.path.iter().filter(|c| c.is_ascii_lowercase()).map(|c| *c))
  }
  fn get_doors(&self) -> HashSet<char> {
    HashSet::from_iter(self.path.iter().filter(|c| c.is_ascii_uppercase()).map(|c| *c))
  }
  fn get_open_doors(&self) -> HashSet<char> {
    HashSet::from_iter(self.path.iter().filter(|c| c.is_ascii_lowercase()).map(|c| c.to_ascii_uppercase()))
  }
  fn get_nodes(&self) -> HashSet<char> {
    HashSet::from_iter(self.path.iter().map(|c| *c))
  }
  fn get_last(&self) -> &Vec<char> {
    &self.last
  }
  fn hash(&self) -> String {
    return self.path.iter().collect();
  }
  fn add_node(&self, who: usize, node: char, cost: usize) -> Self {
    let mut new_path = self.path.to_vec();
    let mut new_last = self.last.to_vec();
    new_path.push(node);
    new_last[who] = node;
    return Self {
      path: new_path,
      last: new_last,
      cost: self.cost + cost
    }
  }
}

//// (END) PATH AND COST (END) ////

fn get_elements(grid : &Grid) -> HashMap<char,Point> {
  let mut ret = HashMap::new();
  let (min_x, max_x, min_y, max_y) = grid.minmax();
  let mut alt_sign_at : Vec<char> = vec!['$','&','%','@'];
  for y in min_y..(max_y+1) {
    for x in min_x..(max_x+1) {
      let pos = (x,y);
      let c = grid.get(&pos);
      match c {
        'a'..='z' | 'A'..='Z' => assert!(ret.insert(c, pos).is_none()),
        '@' => assert!(ret.insert(alt_sign_at.pop().unwrap(), pos).is_none()),
        _ => ()
      }
    }
  }
  return ret;
}

fn get_reachable(grid : &Grid, start : &Point) -> Vec<(char,usize)>{
  let mut reachable = Vec::new();
  let mut open : VecDeque<Point> = VecDeque::new();
  let mut close : HashMap<Point,usize> = HashMap::new();
  close.insert(*start, 0);
  for nb in &NEIGHBOR {
    let npos = (start.0 + nb.0, start.1 + nb.1);
    if !close.contains_key(&npos) {
      close.insert(npos, 1);
      open.push_back(npos);
    }
  }
  while !open.is_empty() {
    let pos = open.pop_front().expect("checked above");
  let cost = *close.get(&pos).expect("all position in open should have a cost");
    let c = grid.get(&pos);
    match c {
      '@' | '%' | '$' | '&' | 'a'..='z' | 'A'..='Z'  => reachable.push((c, cost)),
      _ => ()
    }
    match c {
      '@' | '%' | '$' | '&' | 'a'..='z' | '.' => {
        for nb in &NEIGHBOR {
          let npos = (pos.0 + nb.0, pos.1 + nb.1);
          if !close.contains_key(&npos) {
            close.insert(npos, cost + 1);
            open.push_back(npos);
          }
        }
      },
      _ => ()
    }
  }
  return reachable;
}

fn build_graph(grid: &Grid, nodes: &HashMap<char,Point>) -> HashMap<char, HashMap<char,usize>> {
  let mut graph = HashMap::new();
  for (key, val) in nodes {
    graph.insert(*key, HashMap::from_iter(get_reachable(grid, val)));
  }
  return graph;
}

fn get_path_from_key_to_key(from: char, to: char, graph: &HashMap<char, HashMap<char,usize>>) -> Vec<PathAndCost> {
  fn do_hash(doors: &HashSet<char>) -> String {
    let mut tmp : Vec<&char> = doors.into_iter().collect();
    tmp.sort();
    return tmp.into_iter().collect();
  }
  fn is_one_solution_better(solutions: &Vec<PathAndCost>, doors: &HashSet<char>) -> bool {
    solutions.iter().any(|s| !s.path.iter().filter(|c| c.is_ascii_uppercase()).any(|d| !doors.contains(d)))
  //  solutions.iter().any(|s| !s.get_doors().iter().any(|d| !doors.contains(d)))
  }
  let mut solutions = Vec::new();

  let mut open : BinaryHeap<PathAndCost> = BinaryHeap::new();
  open.push(PathAndCost { path: vec![from], cost: 0});
  while !open.is_empty() {
    let cur = open.pop().unwrap();
    let nodes = cur.get_nodes();
    let doors = cur.get_doors();
    if is_one_solution_better(&solutions, &doors) { continue; } // We already know a better solution using the same doors or less
    
    let neighbor = graph.get(&cur.get_last()).expect("node missing from the graph");
    for (nb, cost) in neighbor {
      if nodes.contains(nb) { continue; } // avoid loop
      if *nb == to { // we've got a solution
        solutions.push(cur.add_node(*nb, *cost));
        break;
      }
      if !nb.is_ascii_uppercase() { continue; } // no need to stop on other keys
      open.push(cur.add_node(*nb, *cost)); // all good, register info and save the state
    }
  }

  return solutions;
}

fn build_key_graph(graph: &HashMap<char, HashMap<char,usize>>) -> HashMap<char,Vec<(char,HashSet<char>,usize)>> {
  let mut ret : HashMap<char,Vec<(char,HashSet<char>,usize)>> = HashMap::new();
  let keys : Vec<char> = graph.keys().cloned().filter(|c| !c.is_ascii_uppercase()).collect();
  let mut nb_done : i32 = 0;
  for from in keys.to_vec() {
    let mut neighbors: Vec<(char,HashSet<char>,usize)> = Vec::new();
    println!("from: {} {}", from, nb_done as f64 / keys.len() as f64);
    for to in &keys {
      if from == *to { continue; }
      //println!("from: {} to: {}", from, to);
      let solutions : Vec<(char,HashSet<char>,usize)> = get_path_from_key_to_key(from, *to, graph).iter().map(|s| (*to, s.get_doors(), s.cost)).collect();
      neighbors.extend(solutions);
    }
    neighbors.sort_by_key(|n| n.2);
    nb_done = nb_done + 1;
    ret.insert(from, neighbors);
  }
  return ret;
}

fn solve_key_graph(graph : &HashMap<char,Vec<(char,HashSet<char>,usize)>>) -> Vec<PathAndCost> {
  fn do_hash(nodes: &Vec<char>) -> String {
    //return nodes.iter().collect();
    let mut tmp : Vec<char> = nodes.to_vec();
    let last = tmp.pop().unwrap();
    tmp.sort();
    tmp.push(last);
    return tmp.into_iter().collect();
  }
  let mut solutions = Vec::new();
  //println!("{:?}", graph);
  let mut open : BinaryHeap<PathAndCost> = BinaryHeap::new();
  let mut close : HashSet<String> = HashSet::new();
  let nb_keys = graph.keys().filter(|c| c.is_ascii_lowercase()).count();
  let mut step : usize = 0;
  open.push(PathAndCost { path: ['@'].to_vec(), cost: 0});
  while !open.is_empty() {
    let cur = open.pop().unwrap();
    let hash = do_hash(&cur.path);
    if !close.insert(hash) { continue; }
    let open_doors = cur.get_open_doors();
    let nodes = cur.get_nodes();
    let neighbor = graph.get(&cur.get_last()).expect("node missing from the graph");
    for (nb, doors_to_use, cost) in neighbor {
      //println!("from:{} to:{} via:{:?} cost:{}", cur.get_last(), nb, doors_to_use, cost);
      if nodes.contains(nb) { // Don't loop
        //println!("excluded to avoid loop");
        continue;
      }
      if !doors_to_use.is_subset(&open_doors) { // You miss some keys
        //println!("excluded because of a missing key");
        continue;
      }
      // all good, register info and save the state
      let ncur = cur.add_node(*nb,*cost);
      if nodes.len() == nb_keys { // we've got a winner
        solutions.push(ncur);
        //return solutions;
      } else {
        open.push(ncur);
      }
    }
    step = step + 1;
    if step % 100000 == 0 {
      println!("Step: {}", step);
      println!("Current cost: {}", cur.cost);
      println!("Current investigated path: {}", cur.hash());
      println!("Current size of open: {}", open.len());
      println!("Current size of close: {}", close.len());
      println!("Current size of solutions: {}", solutions.len());
    }
  }

  return solutions;
}

fn solve_key_graph_multibot(graph : &HashMap<char,Vec<(char,HashSet<char>,usize)>>) -> Vec<PathAndCostMultiBot> {
  fn do_hash(nodes: &Vec<char>, last: &Vec<char>) -> String {
    //return nodes.iter().collect();
    let mut tmp : Vec<char> = nodes.to_vec();
    tmp.extend(last.to_vec());
    tmp.sort();
    return tmp.into_iter().collect();
  }
  let mut solutions = Vec::new();
  //println!("{:?}", graph);
  let mut open : BinaryHeap<PathAndCostMultiBot> = BinaryHeap::new();
  let mut close : HashSet<String> = HashSet::new();
  let nb_keys = graph.keys().filter(|c| c.is_ascii_lowercase()).count();
  let mut step : usize = 0;
  open.push(PathAndCostMultiBot::initial());
  while !open.is_empty() {
    let cur = open.pop().unwrap();
    let hash = do_hash(&cur.path,&cur.get_last());
    if !close.insert(hash) { continue; }
    let open_doors = cur.get_open_doors();
    let nodes = cur.get_nodes();
    let last = cur.get_last();
    for bot in 0..4 {
      let neighbor = graph.get(&last[bot]).expect("node missing from the graph");
      for (nb, doors_to_use, cost) in neighbor {
        //println!("from:{} to:{} via:{:?} cost:{}", &last[bot], nb, doors_to_use, cost);
        if nodes.contains(nb) { // Don't loop
          //println!("excluded to avoid loop");
          continue;
        }
        match *nb {
          '@' | '%' | '$' | '&' => continue,
          _ => ()
        }
        if !doors_to_use.is_subset(&open_doors) { // You miss some keys
          //println!("excluded because of a missing key");
          continue;
        }
        // all good, register info and save the state
        let ncur = cur.add_node(bot,*nb,*cost);
        if nodes.len()+1 == nb_keys { // we've got a winner
          solutions.push(ncur);
          //return solutions;
        } else {
          open.push(ncur);
        }
      }
      step = step + 1;
      if step % 100000 == 0 {
        println!("Step: {}", step);
        println!("Current cost: {}", cur.cost);
        println!("Current investigated path: {}", cur.hash());
        println!("Current size of open: {}", open.len());
        println!("Current size of close: {}", close.len());
        println!("Current size of solutions: {}", solutions.len());
      }
    }
  }

  return solutions;
}

fn q1(filename: impl AsRef<std::path::Path>) -> (usize, Vec<char>) {
  let grid = Grid::new_from_file(filename);
  println!("grid extracted");
  let nodes = get_elements(&grid);
  println!("nodes retrieved");
  let graph = build_graph(&grid, &nodes);
  println!("graph built");
  let key_graph = build_key_graph(&graph);
  println!("key graph built");
  let solutions = solve_key_graph(&key_graph);
  
  for solution in &solutions {
    println!("solution: {:?} cost: {}", solution.path, solution.cost);
  }
  println!("nb of solutions: {}", solutions.len());
  assert!(solutions.len() > 0);
  let mut best_cost = None;
  let mut best_solution = None;
  for solution in solutions {
    if best_solution.is_none() || solution.cost < best_cost.unwrap() {
      best_cost = Some(solution.cost);
      best_solution = Some(solution.path);
    }
  }
  return (best_cost.unwrap(), best_solution.unwrap());
}

fn modify_the_grid(mut grid : Grid) -> Grid {
  let nodes = get_elements(&grid);
  let pos = nodes.get(&'@').unwrap();
  grid.set((pos.0-1,pos.1-1), '@');
  grid.set((pos.0+0,pos.1-1), '#');
  grid.set((pos.0+1,pos.1-1), '@');
  grid.set((pos.0-1,pos.1+0), '#');
  grid.set((pos.0+0,pos.1+0), '#');
  grid.set((pos.0+1,pos.1+0), '#');
  grid.set((pos.0-1,pos.1+1), '@');
  grid.set((pos.0+0,pos.1+1), '#');
  grid.set((pos.0+1,pos.1+1), '@');
  return grid;
}

fn q2(filename: impl AsRef<std::path::Path>, from_q1: bool) -> (usize, Vec<char>) {
  let mut grid = Grid::new_from_file(filename);
  println!("grid extracted");
  if from_q1 {
    grid = modify_the_grid(grid);
  }
  println!("grid modified");
  let nodes = get_elements(&grid);
  println!("nodes retrieved");
  let graph = build_graph(&grid, &nodes);
  println!("graph built");
  let key_graph = build_key_graph(&graph);
  println!("key graph built");
  let solutions = solve_key_graph_multibot(&key_graph);
  /*
  for solution in &solutions {
    println!("solution: {:?} cost: {}", solution.path, solution.cost);
  }*/
  println!("nb of solutions: {}", solutions.len());
  assert!(solutions.len() > 0);
  let mut best_cost = None;
  let mut best_solution = None;
  for solution in solutions {
    if best_solution.is_none() || solution.cost < best_cost.unwrap() {
      best_cost = Some(solution.cost);
      best_solution = Some(solution.path);
    }
  }
  return (best_cost.unwrap(), best_solution.unwrap());
}

fn main() {
  println!("Question1: {:?}", q1("data.txt"));
  println!("Question2: {:?}", q2("data.txt", true));
}

#[test]
fn test_q1_test1() {
  assert_eq!(q1("test1.txt").0, 8);
}

#[test]
fn test_q1_test2() {
  assert_eq!(q1("test2.txt").0, 86);
}

#[test]
fn test_q1_test3() {
  assert_eq!(q1("test3.txt").0, 132);
}

#[test]
fn test_q1_test4() {
  assert_eq!(q1("test4.txt").0, 136);
}

#[test]
fn test_q1_test5() {
  assert_eq!(q1("test5.txt").0, 81);
}

#[test]
fn test_q2_test1() {
  assert_eq!(q2("test6.txt", true).0, 8);
}

#[test]
fn test_q2_test2() {
  assert_eq!(q2("test7.txt", false).0, 24);
}

#[test]
fn test_q2_test3() {
  assert_eq!(q2("test8.txt", false).0, 32);
}

#[test]
fn test_q2_test4() {
  assert_eq!(q2("test9.txt", false).0, 72);
}