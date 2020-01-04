#![allow(dead_code,unused_imports)]

use std::fs;
use std::collections::HashSet;

#[derive(Debug,PartialEq, Eq, Hash, Copy, Clone)]
enum Instruction {
  DealIntoNewStack,
  DealWithIncrement(i64),
  Cut(i64)
}

fn mulmod(mut a: i64, mut b: i64, m: usize) -> usize {
  let mut res : i64 = 0;
  if a < 0 { a = a + m as i64; }
  if b < 0 { b = b + m as i64; }
  a = a % (m as i64);
  b = b % (m as i64);
  while b > 0 {
    if b % 2 == 1 { 
      res = (res + a) % (m as i64);
    }
    a = (a * 2) % (m as i64);
    b = b / 2;
  }
  if res < 0 { res = (m as i64) + res; }
  return (res as usize) % m;
}

fn addmod(a: i64, b: i64, m: usize) -> usize {
  let mut res : i64 = a + b;
  if res < 0 { res = res + m as i64; }
  return (res as usize) % m;  
}

fn parse(filename: impl AsRef<std::path::Path>) -> Vec<Instruction> {
  let mut ret = Vec::new();
  for line in fs::read_to_string(filename).expect("Something went wrong reading the file").lines() {
    if line.starts_with("deal into new stack") {
      ret.push(Instruction::DealIntoNewStack);
    } else if line.starts_with("deal with increment") {
      ret.push(Instruction::DealWithIncrement(line.split(" ").last().unwrap().parse::<i64>().unwrap()));
    } else if line.starts_with("cut") {
      ret.push(Instruction::Cut(line.split(" ").last().unwrap().parse::<i64>().unwrap()));
    } else {
      panic!("unexpected line: {}", line);      
    }    
  }
  return ret;
}

fn replace_all_deal_into_stack(instructions: &Vec<Instruction>, _deck_size: usize) -> Vec<Instruction> {
  let mut ret : Vec<Instruction> = Vec::new();
  for instruction in instructions {
    if *instruction == Instruction::DealIntoNewStack {
      ret.push(Instruction::DealWithIncrement(-1));
      ret.push(Instruction::Cut(1));
    } else {
      ret.push(*instruction);
    }
  }  
  return ret;
}

fn bring_up_all_deal_with_increment(instructions: &Vec<Instruction>, deck_size: usize) -> Vec<Instruction> {
  let mut prev : Option<Instruction> = None;
  let mut ret : Vec<Instruction> = Vec::new();
  for instruction in instructions {
    prev = Some(
      match prev {
        None => *instruction,
        Some(prev_instruction) => {
          match prev_instruction {
            Instruction::Cut(a) => {
              match *instruction {
                Instruction::DealWithIncrement(b) => {
                  ret.push(*instruction);
                  Instruction::Cut(mulmod(a,b,deck_size) as i64)
                },
                Instruction::Cut(b) => {
                  Instruction::Cut(addmod(a,b,deck_size) as i64)
                },
                _ => {
                  ret.push(prev_instruction);
                  *instruction
                }
              }
            },
            _ => {
              ret.push(prev_instruction);
              *instruction
            }
          }
        }
      }
    );
  }
  ret.push(prev.unwrap());
  return ret;
}

fn reduce_all_consecutive_instruction(instructions: &Vec<Instruction>, deck_size: usize) -> Vec<Instruction> {
  let mut prev : Option<Instruction> = None;
  let mut ret : Vec<Instruction> = Vec::new();
  for instruction in instructions {
    prev = Some(
      match prev {
        None => *instruction,
        Some(prev_instruction) => {
          match prev_instruction {
            Instruction::DealWithIncrement(a) => {
              match *instruction {
                Instruction::DealWithIncrement(b) => {
                  Instruction::DealWithIncrement(mulmod(a,b,deck_size) as i64)
                },
                _ => {
                  ret.push(prev_instruction);
                  *instruction
                }
              }
            },
            _ => {
              ret.push(prev_instruction);
              *instruction
            }
          }
        }
      }
    );
  }
  ret.push(prev.unwrap());
  return ret;
}


fn reduce_n_stack(instructions: &Vec<Instruction>, mut nb_time: usize, deck_size: usize) -> Vec<Instruction> {
  let add = |a: (i64,i64), b: (i64,i64)| (mulmod(a.0,b.0,deck_size) as i64,addmod(mulmod(a.1,b.0,deck_size) as i64,b.1,deck_size) as i64);
  let mul2 = |a: (i64,i64)| (mulmod(a.0,a.0,deck_size) as i64,addmod(mulmod(a.1,a.0,deck_size) as i64,a.1,deck_size) as i64);

  let mut reduce_instr = replace_all_deal_into_stack(&instructions, deck_size);
  reduce_instr = bring_up_all_deal_with_increment(&reduce_instr, deck_size);
  reduce_instr = reduce_all_consecutive_instruction(&reduce_instr, deck_size);
  assert_eq!(reduce_instr.len(),2);
  let incr = match reduce_instr[0] {
    Instruction::DealWithIncrement(a) => a,
    _ => panic!("first instruction should be a dealWithIncrement")
  };
  let cut_pos = match reduce_instr[1] {
    Instruction::Cut(a) => a,
    _ => panic!("second instruction should be a cut")
  };
  
  let mut res : (i64,i64) = (1,0);
  let mut acc : (i64,i64) = (incr, cut_pos);
  while nb_time > 0 {
    if nb_time % 2 == 1 {
      res = add(res, acc);
    }
    acc = mul2(acc);
    nb_time = nb_time / 2;
  }
  return vec![Instruction::DealWithIncrement(res.0), Instruction::Cut(res.1)];
}

fn deal_into_new_stack(mut deck: Vec<usize>) -> Vec<usize> {
  deck.reverse();
  return deck;
}

fn deal_with_increment(deck: Vec<usize>, incr: i64) -> Vec<usize> {
  let n = deck.len();
  let mut ret = vec![0; n];
  for i in 0..n {
    println!("{}", mulmod(i as i64,incr,n));
    ret[mulmod(i as i64,incr,n)] = deck[i];
  }
  return ret;
}

fn cut(mut deck: Vec<usize>, pos: i64) -> Vec<usize> {
  if pos == 0 { return deck; }
  let pivot : usize;
  if pos > 0 {
    pivot = pos as usize;
  } else {
    pivot = (deck.len() as i64 + (pos as i64)) as usize;
  }
  let mut right = deck.split_off(pivot);
  right.extend(deck);
  return right;
}

fn deal_into_new_stack_pos(p: usize, deck_size: usize) -> usize {
  return (deck_size as i64 - 1 - p as i64) as usize;
}

fn deal_with_increment_pos(p: usize, deck_size: usize, incr: i64) -> usize {
  return mulmod(p as i64, incr, deck_size);
  //return (p*incr)%deck_size;
}

pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
  if a == 0 {
    (b, 0, 1)
  } else {
    let (g, x, y) = egcd(b % a, a);
    (g, y - (b / a) * x, x)
  }
}

fn deal_with_increment_inv_pos(p: usize, deck_size: usize, incr: usize) -> usize {
  let (g, n, m) = egcd(incr as i64, deck_size as i64);
  return mulmod(p as i64, n as i64, deck_size);
}

/*
fn deal_with_increment_inv_pos(p: usize, deck_size: usize, incr: usize) -> usize {
  println!("p: {}, deck_size: {}, incr: {}", p, deck_size, incr);
  let n : usize = deck_size/incr;
  let m : usize = deck_size%incr;
  let v : usize = p%incr;
  let mut d : usize = 0;
  let mut i : usize = 0;
  let mut off : usize = 0;
  println!("m: {}, n: {}, v: {}", m, n, v);
  while v != d {
    off = off + n + if d <= m { 1 } else { 0 };
    d = ((d+(n+1)*incr)-deck_size) % incr;
    i = i + 1;
    assert!(d<incr);
    println!("d: {} v: {} i: {}", d, v, i);
    assert!(i<incr);
  }
  return off + (p-v)/incr;
}
*/
fn cut_pos(p: usize, deck_size: usize, pos: i64) -> usize {
  if pos == 0 { return p; }
  let pivot : usize;
  if pos > 0 {
    pivot = pos as usize;
  } else {
    pivot = (deck_size as i64 + (pos as i64)) as usize;
  }
  if p >= pivot {
    return p - pivot;
  }
  return (deck_size - pivot) + p;
}

fn shuffle(mut deck: Vec<usize>, instructions: Vec<Instruction>) -> Vec<usize> {
  for instruction in instructions {
    deck = match instruction {
      Instruction::DealIntoNewStack => deal_into_new_stack(deck),
      Instruction::DealWithIncrement(incr) => deal_with_increment(deck, incr),
      Instruction::Cut(pos) => cut(deck, pos),
    }
  }
  return deck;
}

fn shuffle_pos(mut p: usize, deck_size: usize, instructions: &Vec<Instruction>) -> usize {
  for instruction in instructions {
    p = match instruction {
      Instruction::DealIntoNewStack => deal_into_new_stack_pos(p, deck_size),
      Instruction::DealWithIncrement(incr) => deal_with_increment_pos(p, deck_size, *incr),
      Instruction::Cut(pos) => cut_pos(p, deck_size, *pos),
    }
  }
  return p;
}

fn shuffle_inv_pos(mut p: usize, deck_size: usize, instructions: &Vec<Instruction>) -> usize {
  for instruction in instructions.iter().rev() {
    p = match instruction {
      Instruction::DealIntoNewStack => deal_into_new_stack_pos(p, deck_size),
      Instruction::DealWithIncrement(incr) => deal_with_increment_inv_pos(p, deck_size, *incr as usize), //TODO: hum, incr can be negative now
      Instruction::Cut(pos) => cut_pos(p, deck_size, -*pos),
    }
  }
  return p;
}

fn parse_and_shuffle(filename: impl AsRef<std::path::Path>, deck_size: usize) -> Vec<usize> {
  let mut deck = Vec::new();
  for i in 0..deck_size { deck.push(i); }
  return shuffle(deck, parse(filename));
}

fn found(deck: &Vec<usize>, card: usize) -> usize {
  for i in 0..deck.len() {
    if deck[i] == card {
      return i;
    }
  }
  panic!("unable to found card {}", card);
}

fn q1(filename: impl AsRef<std::path::Path>) -> usize {
  return found(&parse_and_shuffle(filename,10007),2019);
}

fn q1_pos(filename: impl AsRef<std::path::Path>) -> usize {
  return shuffle_pos(2019, 10007, &parse(filename));
}
/*
fn q2_fwd(filename: impl AsRef<std::path::Path>) -> usize {
  let nb_time: usize = 101741582076661;
  // 101741582076661
  let deck_size = 119315717514047;
  let mut nb_step : usize = 0;
  let mut pos = 2020;
  let mut already_seen : HashSet<usize> = HashSet::new();
  let instr = parse(filename);
  loop {
    pos = shuffle_pos(pos, deck_size, &instr);
    nb_step = nb_step + 1;
    if !already_seen.insert(pos) {
      println!("nb step: {}", nb_step);
      break;
    }
    if pos == 2020 {
      println!("nb step: {}", nb_step);
      break;
    }
    //println!("{}", pos);
    if nb_step % 100000000 == 0 {
      println!("nb step: {}", nb_step);
    }
  }
  return 5;
}

fn q2_bckwd(filename: impl AsRef<std::path::Path>) -> usize {
  let nb_time : usize = 101741582076661;
  let deck_size : usize = 119315717514047;
  let mut nb_step : usize = 0;
  let mut pos : usize = 2020;
  let mut already_seen : HashSet<usize> = HashSet::new();
  let instr = parse(filename);
  

  loop {
    pos = shuffle_inv_pos(pos, deck_size, &instr);
    nb_step = nb_step + 1;
    if !already_seen.insert(pos) {
      println!("nb step: {}", nb_step);
      break;
    }
    if pos == 2020 {
      println!("nb step: {}", nb_step);
      break;
    }
    if nb_step % 10000000 == 0 {
      println!("nb step: {}", nb_step);
    }
  }
  return 5;
}
*/
fn q2(filename: impl AsRef<std::path::Path>) -> usize {
  let nb_time : usize = 101741582076661;
  let deck_size : usize = 119315717514047;
    
  let instr = parse(filename);
  println!("reduce_n_stack");
  let reduce_instr = reduce_n_stack(&instr, nb_time, deck_size);
  println!("shuffle_inv_pos");
  return shuffle_inv_pos(2020, deck_size, &reduce_instr);
}

fn main() {
    //println!("q1 old method: {}", q1("data.txt"));
    //println!("q1 pos method: {}", q1_pos("data.txt"));
    println!("q2: {}", q2("data.txt"));
}

#[test]
fn test_q1_examples1() {
  assert_eq!(parse_and_shuffle("test1.txt",10), [0,3,6,9,2,5,8,1,4,7]);
}

#[test]
fn test_q1_examples2() {
  assert_eq!(parse_and_shuffle("test2.txt",10), [3,0,7,4,1,8,5,2,9,6]);
}

#[test]
fn test_q1_examples3() {
  assert_eq!(parse_and_shuffle("test3.txt",10), [6,3,0,7,4,1,8,5,2,9]);
}

#[test]
fn test_q1_examples4() {
  assert_eq!(parse_and_shuffle("test4.txt",10), [9,2,5,8,1,4,7,0,3,6]);
}

#[test]
fn test_deal_incr_inverse_small() {
  let mut pos : usize = 0;
  
  pos = deal_with_increment_pos(3, 10, 3);
  assert_eq!(pos, 9);
  pos = deal_with_increment_inv_pos(pos, 10, 3);
  assert_eq!(pos, 3);
  
  pos = deal_with_increment_pos(4, 10, 3);
  assert_eq!(pos, 2);
  pos = deal_with_increment_inv_pos(pos, 10, 3);
  assert_eq!(pos, 4);
  
  pos = deal_with_increment_pos(7, 10, 3);
  assert_eq!(pos, 1);
  pos = deal_with_increment_inv_pos(pos, 10, 3);
  assert_eq!(pos, 7);
}

#[test]
fn test_inverse_small_pos_huge_deck() {
  let deck_size : usize = 119315717514047;
  let test : usize = 2020;
  let mut pos : usize = 0;
  
  pos = deal_into_new_stack_pos(test, deck_size);
  pos = deal_into_new_stack_pos(pos, deck_size);
  assert_eq!(pos, test);
  
  pos = deal_with_increment_pos(test, deck_size, 52);
  pos = deal_with_increment_inv_pos(pos, deck_size, 52);
  assert_eq!(pos, test);
  
  pos = cut_pos(test, deck_size, 100);
  pos = cut_pos(pos, deck_size, -100);
  assert_eq!(pos, test);
}

#[test]
fn test_inverse_huge_pos_huge_deck() {
  let deck_size : usize = 119315717514047;
  let test : usize = 119315717471626;
  let mut pos : usize = 0;
  
  pos = deal_into_new_stack_pos(test, deck_size);
  pos = deal_into_new_stack_pos(pos, deck_size);
  assert_eq!(pos, test);
  
  pos = deal_with_increment_pos(test, deck_size, 52);
  pos = deal_with_increment_inv_pos(pos, deck_size, 52);
  assert_eq!(pos, test);
  
  pos = cut_pos(test, deck_size, 100);
  pos = cut_pos(pos, deck_size, -100);
  assert_eq!(pos, test);
}

#[test]
fn test_inverse_pos_huge() {
  let deck_size : usize = 119315717514047;
  let test : usize = 56283179575553;
  let mut pos : usize = 0;
  
  pos = deal_with_increment_pos(test, deck_size, 72);
  pos = deal_with_increment_inv_pos(pos, deck_size, 72);
  assert_eq!(pos, test);
}

#[test]
fn test_reduction_remove_deal_into_new_stack() {
  let deck_size : usize = 119315717514047;
  let test : usize = 56283179575553;
  
  let pos1 = deal_into_new_stack_pos(test, deck_size);
  
  let mut pos2 = deal_with_increment_pos(test, deck_size, -1);
  pos2 = cut_pos(pos2, deck_size, 1);
  
  assert_eq!(pos1, pos2);
}

#[test]
fn test_switch_cut_and_increment() {
  let deck_size : usize = 119315717514047;
  let test : usize = 56283179575553;
  
  let mut pos1 = cut_pos(test, deck_size, 5);
  pos1 = deal_with_increment_pos(pos1, deck_size, 1000);
  
  let mut pos2 = deal_with_increment_pos(test, deck_size, 1000);
  pos2 = cut_pos(pos2, deck_size, 5000);
  
  assert_eq!(pos1, pos2);
}

#[test]
fn test_acc_cut() {
  let deck_size : usize = 119315717514047;
  let test : usize = 56283179575553;
  
  let mut pos1 = cut_pos(test, deck_size, 5);
  pos1 = cut_pos(pos1, deck_size, 1000);
  
  let mut pos2 = cut_pos(test, deck_size, 1005);
  
  assert_eq!(pos1, pos2);
}

#[test]
fn test_acc_incr() {
  let deck_size : usize = 119315717514047;
  let test : usize = 56283179575553;
  
  let mut pos1 = deal_with_increment_pos(test, deck_size, 5);
  pos1 = deal_with_increment_pos(pos1, deck_size, 1000);
  
  let mut pos2 = deal_with_increment_pos(test, deck_size, 5000);
  
  assert_eq!(pos1, pos2);
}

#[test]
fn test_reduction_remove_deal_into_new_stack_small() {
  let deck_size : usize = 13;
  
  let mut deck1 = Vec::new();
  let mut deck2 = Vec::new();
  for i in 0..deck_size { 
    deck1.push(i);
    deck2.push(i);
  }
  
  deck1 = deal_into_new_stack(deck1);
  
  deck2 = deal_with_increment(deck2, -1);
  println!("{:?}", deck2);
  deck2 = cut(deck2, 1);
  
  assert_eq!(deck1, deck2);
}

#[test]
fn test_reduction() {
  let deck_size : usize = 119315717514047;
  let init : usize = 2020;
  let mut pos : usize = init;
  
  let instr = parse("data.txt");
  println!("original[{}]:\n {:?}", instr.len(), instr);
  let test1 = shuffle_pos(pos, deck_size, &instr);
  let test1_x3 = shuffle_pos(shuffle_pos(test1, deck_size, &instr), deck_size, &instr);
  
  let mut reduce_instr = replace_all_deal_into_stack(&instr, deck_size);
  println!("remove deal into[{}]:\n {:?}", reduce_instr.len(), reduce_instr);  
  let test2 = shuffle_pos(pos, deck_size, &reduce_instr);
  assert_eq!(test1, test2);
  
  reduce_instr = bring_up_all_deal_with_increment(&reduce_instr, deck_size);
  println!("sort[{}]:\n {:?}", reduce_instr.len(), reduce_instr);
  let test3 = shuffle_pos(pos, deck_size, &reduce_instr);
  assert_eq!(test1, test3);
  
  reduce_instr = reduce_all_consecutive_instruction(&reduce_instr, deck_size);  
  println!("final[{}]:\n {:?}", reduce_instr.len(), reduce_instr);
  let test4 = shuffle_pos(pos, deck_size, &reduce_instr);
  assert_eq!(test1, test4);
  
  let reduce_instr_x3 = reduce_n_stack(&reduce_instr, 3, deck_size);
  let test2_x3 = shuffle_pos(init, deck_size, &reduce_instr_x3);
  assert_eq!(test1_x3, test2_x3);
}

#[test]
fn test_reduction_n_time() {
  let deck_size : usize = 119315717514047;
  let init : usize = 2020;
  let mut pos : usize = init;
  
  let instr = parse("data.txt");
  let test1_x3 = shuffle_pos(shuffle_pos(shuffle_pos(pos, deck_size, &instr), deck_size, &instr), deck_size, &instr);
  let reduce_instr_x3 = reduce_n_stack(&instr, 3, deck_size);
  let test2_x3 = shuffle_pos(init, deck_size, &reduce_instr_x3);
  assert_eq!(test1_x3, test2_x3);
}