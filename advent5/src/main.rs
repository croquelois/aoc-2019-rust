use std::fs;

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

fn process(mem: &mut Vec<i32>, grab_input: &dyn Fn() -> i32, send_output: &dyn Fn(i32) -> ()) {
  let mut ip = 0;
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
        mem[p] = grab_input();
        ip = ip + 2;
      }
      4 => {
        send_output(get_param(0, ip, &mem));
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
      99 => break,
      _ => panic!("unexpected operator"),
    }
  }
}

fn q1() -> () {
  let mut mem = parse("data.txt");
  process(&mut mem, &|| 1, &|i:i32| println!("{}",i));
}

fn q2() -> () {
  let mut mem = parse("data.txt");
  process(&mut mem, &|| 5, &|i:i32| println!("{}",i));
}

fn main() {
  println!("Question1");
  q1();
  println!("Question2");
  q2();
}
