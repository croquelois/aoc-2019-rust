use std::fs;
use std::collections::HashSet;

fn parse_token(token: &String) -> (String, i32) {
  let dir = token.get(0..1).unwrap().to_string();
  let step = token.get(1..).unwrap().parse::<i32>().unwrap();
  return (dir, step);
}
fn parse_line(line: &String) -> Vec<(String, i32)> {
  return line.split(",").map(|s| parse_token(&s.to_string())).collect();
}

fn parse(filename: impl AsRef<std::path::Path>) -> Vec<Vec<(String, i32)>> {
  return fs::read_to_string(filename).expect("Something went wrong reading the file")
   .lines().map(|s| parse_line(&s.to_string())).collect();
}

fn populate_grid(path: &Vec<(String, i32)>, grid : &mut HashSet<(i32,i32)>) {
  let mut pos_x: i32 = 0;
  let mut pos_y: i32 = 0;
  for section in path {
    let dir_char = &section.0[..];
    let mut dir_x: i32 = 0;
    let mut dir_y: i32 = 0;
    match dir_char {
      "U" => dir_y = 1,
      "D" => dir_y = -1,
      "L" => dir_x = -1,
      "R" => dir_x = 1,
      _ => panic!("unexpected direction")
    }
    let step = section.1;
    for _ in 0..step {
      pos_x += dir_x;
      pos_y += dir_y;
      grid.insert((pos_x,pos_y));
    }
  }
}

fn compute_manhattan(p: &(i32, i32)) -> i32{
  return p.0.abs() + p.1.abs();
}

fn cmp_manhattan(p1: &(i32,i32), p2: &(i32,i32)) -> std::cmp::Ordering {
  let d1 = compute_manhattan(p1);
  let d2 = compute_manhattan(p2);
  return d1.cmp(&d2);
}

fn get_path_intersection(path1: &Vec<(String, i32)>, path2: &Vec<(String, i32)>) -> Vec<(i32,i32)> {
  let mut grid1 = HashSet::new();
  let mut grid2 = HashSet::new();
  populate_grid(path1, &mut grid1);
  populate_grid(path2, &mut grid2);
  return grid1.intersection(&grid2).map(|t| (t.0,t.1)).collect::<Vec<(i32,i32)>>();
}

fn get_inter_cost(path: &Vec<(String, i32)>, inter: &(i32,i32)) -> i32 {
  let mut pos_x: i32 = 0;
  let mut pos_y: i32 = 0;
  let mut cost: i32 = 0;
  for section in path {
    let dir_char = &section.0[..];
    let mut dir_x: i32 = 0;
    let mut dir_y: i32 = 0;
    match dir_char {
      "U" => dir_y = 1,
      "D" => dir_y = -1,
      "L" => dir_x = -1,
      "R" => dir_x = 1,
      _ => panic!("unexpected direction")
    }
    let step = section.1;
    for _ in 0..step {
      pos_x += dir_x;
      pos_y += dir_y;
      cost += 1;
      if pos_x == inter.0 && pos_y == inter.1 {
        return cost;
      }
    }
  }
  panic!("intersection not found")
}

fn q2(filename: &str) -> i32{
  let paths = parse(filename);
  let inter = get_path_intersection(&paths[0], &paths[1]);
  let cost_inter1 = (&inter).into_iter().map(|itr| get_inter_cost(&paths[0], &itr));
  let cost_inter2 = (&inter).into_iter().map(|itr| get_inter_cost(&paths[1], &itr));
  let mut cost_inter : Vec<i32> = cost_inter1.zip(cost_inter2).map(|c| c.0+c.1).collect();
  cost_inter.sort();
  return cost_inter[0];
}

fn q1(filename: &str) -> i32{
  let paths = parse(filename);
  let mut inter = get_path_intersection(&paths[0], &paths[1]);
  inter.sort_by(cmp_manhattan);
  return compute_manhattan(&inter[0]);
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question1: {}", q2("data.txt"));
}

#[test]
fn test_parse_token() {
  let ret = parse_token(&("R150".to_string()));
  assert_eq!(ret.0, "R");
  assert_eq!(ret.1, 150);
}

#[test]
fn test_examples1() {
  assert_eq!(q1("test1.txt"), 6);
  assert_eq!(q1("test2.txt"), 159);
  assert_eq!(q1("test3.txt"), 135);
}

#[test]
fn test_examples2() {
  assert_eq!(q2("test1.txt"), 30);
  assert_eq!(q2("test2.txt"), 610);
  assert_eq!(q2("test3.txt"), 410);
}