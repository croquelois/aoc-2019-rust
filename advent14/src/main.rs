use std::fs;
//use std::collections::HashSet;
use std::collections::HashMap;
//use std::collections::VecDeque;

fn parse_num_token(num_token: &str) -> (i32,String) {
  let mut part = num_token.split(" ");
  let num = part.next().expect("a text should be present")
              .parse::<i32>().expect("a number is expected");
  let txt = part.next().expect("a token should be present").to_string();
  return (num,txt);
}

fn parse_line(line: &str) -> (Vec<(i32,String)>,(i32,String)) {
  let mut part = line.split(" => ");
  let left_str = part.next().expect("a left part should be present").to_string();
  let right_str = part.next().expect("a right part should be present").to_string();
  let left = left_str.split(", ").map(|s| parse_num_token(&s)).collect();
  let right = parse_num_token(&right_str);
  return (left,right);
}

type Formulas = HashMap<String, (i32, Vec<(i32,String)>)>;

fn parse(filename: impl AsRef<std::path::Path>) -> Formulas {
  let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let mut dico = HashMap::new();
  for line in data.lines()
  {
    let tmp = parse_line(line);
    dico.insert((tmp.1).1,((tmp.1).0,tmp.0));
  }
  return dico;
}

fn create(formulas : &Formulas, what: &String, qt_needed : i32, mut available : &mut HashMap<String,i32>, mut ore_cost : &mut i64) -> ()
{
  if what == "ORE"
  {
    *ore_cost = *ore_cost + qt_needed as i64;
    return;
  }

  let mut qt_available = *(available.get(what).unwrap_or(&0));
  qt_available = qt_available - qt_needed;
  available.insert(what.to_string(), qt_available);
  if qt_available < 0
  {
    let qt_list = formulas.get(what).expect("unknow formula bit");
    let list = &qt_list.1;
    let qt_by_reaction = qt_list.0;
    let n = (- qt_available - 1) / qt_by_reaction + 1;
    for elem in list
    {
      create(formulas, &elem.1, n * elem.0, &mut available, &mut ore_cost);
    }
    qt_available = qt_available + n * qt_by_reaction;
    available.insert(what.to_string(), qt_available);
  }
}

fn q1(filename: impl AsRef<std::path::Path>) -> i32 {
  let formulas = parse(filename);
  let mut available = HashMap::new();
  let mut ore_cost = 0;
  create(&formulas, &("FUEL".to_string()), 1, &mut available, &mut ore_cost);
  //println!("available {:?}", available);
  return ore_cost as i32;
}

fn fuel_batch_max_create(formulas : &Formulas, batch: i32, mut available : &mut HashMap<String,i32>, mut ore_cost : &mut i64) -> (i32, HashMap<String,i32>)
{
  let mut nb_created_before = 0;
  let mut ore_cost_before : i64 = 0;
  let mut available_before = available.clone();
  let mut nb_created = 0;
  while *ore_cost < 1000000000000
  {
    available_before = available.clone();
    nb_created_before = nb_created;
    ore_cost_before = *ore_cost;
    create(&formulas, &("FUEL".to_string()), batch, &mut available, &mut ore_cost);
    nb_created = nb_created + batch;
    println!("cost: {}, fuel: {} ", ore_cost, nb_created);
  }
  *ore_cost = ore_cost_before;
  return (nb_created_before, available_before);
}

fn q2(filename: impl AsRef<std::path::Path>) -> i32 {
  let formulas = parse(filename);
  let mut ore_cost = 0;
  let mut nb_created = 0;
  let mut ret = (0,HashMap::new());
  for batch in [1000,100,10,1].iter()
  {
    ret = fuel_batch_max_create(&formulas,*batch as i32,&mut ret.1, &mut ore_cost);
    nb_created = nb_created + ret.0;
    println!("total fuel: {} ", nb_created);    
  }  
  return nb_created;
}

fn main() {
  println!("{:?}", q1("data.txt"));
  println!("{:?}", q2("data.txt"));
}

#[test]
fn test_q1_examples1() {
  assert_eq!(q1("test1.txt"), 13312);
}

#[test]
fn test_q1_examples2() {
  assert_eq!(q1("test2.txt"), 180697);
}

#[test]
fn test_q1_examples3() {
  assert_eq!(q1("test3.txt"), 2210736);
}

#[test]
fn test_q1_examples4() {
  assert_eq!(q1("test4.txt"), 165);
}

#[test]
fn test_q2_examples1() {
  assert_eq!(q2("test1.txt"), 82892753);
}

#[test]
fn test_q2_examples2() {
  assert_eq!(q2("test2.txt"), 5586022);
}

#[test]
fn test_q2_examples3() {
  assert_eq!(q2("test3.txt"), 460664);
}