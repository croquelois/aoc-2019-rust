use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

fn parse(filename: impl AsRef<std::path::Path>) -> Vec<i32> {
  return fs::read_to_string(filename).expect("Something went wrong reading the file")
    .split(",").map(|s| s.parse::<i32>().unwrap()).collect();
}

fn get_param(narg: usize, ip: usize, mem: &Vec<i32>) -> i32 {
    if (mem[ip]/10i32.pow(narg as u32 + 2))%10 == 0 { 
        return mem[mem[ip+1+narg] as usize];
    }
    return mem[ip+1+narg];
}

fn process(ip_and_mem: &mut (usize, Vec<i32>), input: &mut VecDeque<i32>) -> VecDeque<i32> {
  let mut ip = ip_and_mem.0;
  let mem = &mut ip_and_mem.1;
  let mut output : VecDeque<i32> = VecDeque::new();
  loop {
    match mem[ip]%100 {
      1 => {
        let p = mem[ip+3] as usize;
        mem[p] = get_param(0, ip, &mem) + get_param(1, ip, &mem);
        ip = ip + 4;
      }
      2 => {
        let p = mem[ip+3] as usize;
        mem[p] = get_param(0, ip, &mem) * get_param(1, ip, &mem);
        ip = ip + 4;
      }
      3 => {
        let p = mem[ip+1] as usize;
        if input.len() == 0 {
          break;
        }
        mem[p] = input.pop_front().expect("input should not be empty");
        ip = ip + 2;
      }
      4 => {
        let v = get_param(0, ip, &mem);
        output.push_back(v);
        ip = ip + 2;
      }
      5 => {
        if get_param(0, ip, &mem) != 0 {
          ip = get_param(1, ip, &mem) as usize;
        } else {
          ip = ip + 3
        }
      }
      6 => {
        if get_param(0, ip, &mem) == 0 {
          ip = get_param(1, ip, &mem) as usize;
        } else {
          ip = ip + 3
        }
      }
      7 => {
        let p = mem[ip+3] as usize;
        if get_param(0, ip, &mem) < get_param(1, ip, &mem) {
          mem[p] = 1;
        } else {
          mem[p] = 0;
        }
        ip = ip + 4;
      }
      8 => {
        let p = mem[ip+3] as usize;
        if get_param(0, ip, &mem) == get_param(1, ip, &mem) {
          mem[p] = 1;
        } else {
          mem[p] = 0;
        }
        ip = ip + 4;
      }
      99 => {
        ip = mem.len();
        break;
      }
      _ => panic!("unexpected operator"),
    }
  }
  ip_and_mem.0 = ip;
  return output;
}

fn oracle(orig: &Vec<i32>, phases: &Vec<i32>) -> i32 {
  let mut output = VecDeque::new();
  output.push_back(0);
  for phase in phases {
    let mut input : VecDeque<i32> = VecDeque::new();
    input.push_back(*phase);
    input.append(&mut output);
    let mem = orig.clone();
    output = process(&mut (0,mem), &mut input);
  }
  return *output.iter().next().expect("Should not be empty");
}

fn oracle2(orig: &Vec<i32>, phases: &Vec<i32>) -> i32 {
  let mut output = VecDeque::new();
  let mut mem_map : HashMap<i32, (usize,Vec<i32>)> = HashMap::new();
  for phase in phases {
    mem_map.insert(*phase, (0,orig.clone()));
  }
  output.push_back(0);
  let mut start = true;
  loop {
    let mut finished = true;
    for phase in phases {
      let mut input : VecDeque<i32> = VecDeque::new();
      if start {
        input.push_back(*phase);
      }
      let mut ip_and_mem = mem_map.get_mut(phase).expect("Should exist");
      input.append(&mut output);
      output = process(&mut ip_and_mem, &mut input);
      if ip_and_mem.1.len() != ip_and_mem.0 {
        finished = false;
      }
    }
    start = false;
    if finished {
      break;
    }
  }
  return *output.iter().next().expect("Should not be empty");
}

fn rec_gen_phases(possible: &HashSet<i32>) -> Vec<Vec<i32>> {
  let mut ret : Vec<Vec<i32>> = Vec::new();
  if possible.len() == 1 {
    let mut new_vec : Vec<i32> = Vec::new();
    new_vec.push(*possible.iter().next().expect("should not be empty"));
    ret.push(new_vec);
  } else {
    for one in possible {
      let mut new_possible = possible.clone();
      new_possible.remove(one);
      let tmp = rec_gen_phases(&new_possible);
      for mut t in tmp {
        t.push(*one);
        ret.push(t);
      }
    } 
  }
  return ret;
}

fn generate_phases(vec_of_possible : Vec<i32>) -> Vec<Vec<i32>> {
  let mut possible : HashSet<i32> = HashSet::new();
  for i in vec_of_possible {
    possible.insert(i);
  }
  return rec_gen_phases(&possible);
}

fn q1(filename: impl AsRef<std::path::Path>) -> i32 {
  let orig = parse(filename);
  let mut best = 0;
  let all_phases = generate_phases([0,1,2,3,4].to_vec());
  for phases in all_phases {
    let out = oracle(&orig,&phases);
    if out > best {
      println!("new best {:?} : {}",phases,out);
      best = out;
    }
  }
  return best;
}

fn q2(filename: impl AsRef<std::path::Path>) -> i32 {
  let orig = parse(filename);
  let mut best = 0;
  let all_phases = generate_phases([5,6,7,8,9].to_vec());
  for phases in all_phases {
    let out = oracle2(&orig,&phases);
    if out > best {
      println!("new best {:?} : {}",phases,out);
      best = out;
    }
  }
  return best;
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question1: {}", q2("data.txt"));
}

#[test]
fn test_examples1() {
  assert_eq!(q1("test1.txt"), 43210);
  assert_eq!(q1("test2.txt"), 54321);
  assert_eq!(q1("test3.txt"), 65210);
}

#[test]
fn test_examples2() {
  assert_eq!(q2("test4.txt"), 139629729);
  assert_eq!(q2("test5.txt"), 18216);
}

#[test]
fn test_examples3() {
  let orig = parse("test4.txt");
  assert_eq!(oracle2(&orig,&[9,8,7,6,5].to_vec()), 139629729);
}