#![allow(dead_code,unused_imports)]
use std::fs;
use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::ops::Deref;
use rand::prelude::*;
//use rand::seq::SliceRandom; // 0.6.5

const DIRECTION : [&'static str; 4] = ["north","south","east","west"];

fn random_sample<A, T>(iter: A) -> Option<T> where A: Iterator<Item = T> {
    let mut rng = rand::thread_rng();
    let mut elem = None;
    let mut i = 1f64;
    for new_item in iter {
        if rng.gen::<f64>() < (1f64/i) {
            elem = Some(new_item);
        }
        i += 1.0;
    }
    elem
}

#[derive(Debug)]
struct State {
  ip: usize, // instruction pointer
  rb: i64, // relative base
  mem: HashMap<usize,i64>,
  finished: bool,
}

type Point = (i64,i64);

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

  fn set(&mut self, pos: Point, chr: char) {
    self.data.insert(pos, chr);    
  }

  fn get(&self, pos : &Point) -> char {
    return *self.data.get(pos).unwrap_or(&self.dft);
  }

  fn print_minmax(&self, min_x: i64, max_x: i64, min_y:i64, max_y:i64) {
    for y in min_y..(max_y+1) {
      let mut line = Vec::new();
      for x in min_x..(max_x+1) {
        line.push(self.get(&(x,y)));
      }
      let line_str : String = line.into_iter().collect();
      println!("{}", line_str);
    }
  }

  fn minmax(&self) -> (i64, i64, i64, i64) {
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

fn parse_string(data: &String) -> Vec<i64> {
  return data.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
}

fn parse(filename: impl AsRef<std::path::Path>) -> Vec<i64> {
  return parse_string(&fs::read_to_string(filename).expect("Something went wrong reading the file"));
}

impl State {
  fn new_from_vector(code: &Vec<i64>) -> State {
    let mut mem = HashMap::new();
    for (pos, elem) in code.iter().enumerate() {
      mem.insert(pos, *elem);
    }
    return State {
      ip: 0,
      rb: 0,
      mem: mem,
      finished: false
    }
  }

  fn new_from_string(data: &String) -> State {
    return State::new_from_vector(&parse_string(data));
  }

  fn new_from_file(filename: impl AsRef<std::path::Path>) -> State {
    return State::new_from_vector(&parse(filename));
  }

  fn get_mem(&self, pos: usize) -> i64 {
    return *self.mem.get(&pos).unwrap_or(&0);
  }

  fn set_mem(&mut self, pos: usize, v: i64) {
    self.mem.insert(pos, v);
  }

  fn set_mem_indirect(&mut self, pos: usize, v: i64) {
    self.mem.insert(self.get_mem(pos) as usize, v);
  }

  fn get_param(&self, narg: usize) -> i64 {
    let mode = (self.get_mem(self.ip)/10i64.pow(narg as u32 + 2))%10;
    let val = self.get_mem(self.ip+1+narg);
    return match mode { 
      0 => self.get_mem(val as usize),
      1 => val,
      2 => self.get_mem((self.rb + val) as usize),
      _ => panic!("unexpected operator"),
    }
  }

  fn set_param(&mut self, narg: usize, val: i64) {
    let mode = (self.get_mem(self.ip)/10i64.pow(narg as u32 + 2))%10;
    let pos = self.get_mem(self.ip+1+narg);
    self.mem.insert(match mode { 
      0 => pos,
      1 => panic!("mode immediate cannot be used for a output parameter"),
      2 => self.rb + pos,
      _ => panic!("unexpected operator"),
    } as usize, val);
  }

  fn inc_ip(&mut self, v: i64) {
    self.ip = self.ip + v as usize;
  }
  
  fn process(&mut self, input: &mut VecDeque<i64>) -> VecDeque<i64> {
    let mut output : VecDeque<i64> = VecDeque::new();
    loop {
      //println!("{:?}", self);
      match self.get_mem(self.ip)%100 {
        1 => {
          self.set_param(2, self.get_param(0) + self.get_param(1));
          self.inc_ip(4);
        }
        2 => {
          self.set_param(2, self.get_param(0) * self.get_param(1));
          self.inc_ip(4);
        }
        3 => {
          if input.len() == 0 { break; }
          self.set_param(0, input.pop_front().expect("input should not be empty"));
          self.inc_ip(2);
        }
        4 => {
          output.push_back(self.get_param(0));
          self.inc_ip(2);
        }
        5 => {
          if self.get_param(0) != 0 {
            self.ip = self.get_param(1) as usize;
          } else {
            self.inc_ip(3);
          }
        }
        6 => {
          if self.get_param(0) == 0 {
            self.ip = self.get_param(1) as usize;
          } else {
            self.inc_ip(3);
          }
        }
        7 => {
          self.set_param(2, if self.get_param(0) < self.get_param(1) { 1 } else { 0 });
          self.inc_ip(4);
        }
        8 => {
          self.set_param(2, if self.get_param(0) == self.get_param(1) { 1 } else { 0 });
          self.inc_ip(4);
        }
        9 => {
          self.rb = self.rb + self.get_param(0);
          self.inc_ip(2);
        }
        99 => {
          self.finished = true;
          break;
        }
        _ => panic!("unexpected operator"),
      }
    }
    return output;
  }
}

fn output_to_grid(output : &VecDeque<i64>) -> (Grid, VecDeque<i64>) {
  let mut grid = Grid::new();
  let mut p : Point = (0,0);
  let mut unprocessed = VecDeque::new();
  for o in output {
    match o {
      10 => {
        p.0 = 0;
        p.1 = p.1 + 1;
      },
      _ => {
        if *o < 256 {
          grid.set(p, *o as u8 as char);
          p.0 = p.0 + 1;
        } else {
          unprocessed.push_back(*o);
        }
      }
    }
  }
  return (grid, unprocessed);
}

#[derive(Debug,PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
  North,
  South,
  East,
  West,  
}

impl Direction {
  fn as_str(&self) -> &'static str {
    match self {
      Direction::North => "north",
      Direction::South => "south",
      Direction::East => "east",
      Direction::West => "west",
    }
  }
  fn from_str(s: &str) -> Self {
    match s {
      "north" => Direction::North,
      "south" => Direction::South,
      "east"  => Direction::East,
      "west"  => Direction::West,
      _ => panic!("incorrect direction: '{}'", s)
    }
  }
  fn invert(&self) -> Self {
    match self {
      Direction::North => Direction::South,
      Direction::South => Direction::North,
      Direction::East => Direction::West,
      Direction::West => Direction::East,
    }
  }
}

type RoomName = String;
type ItemName = String;

#[derive(Debug)]
struct Room {
  name: RoomName,
  from: Option<(Direction, RoomName)>,
  description: String,
  doors: HashMap<Direction, Option<RoomName>>,
  items: HashSet<ItemName>
}

impl Room {
  fn new_from_output(output : &mut VecDeque<i64>) -> VecDeque<Self> {
    let mut ret : VecDeque<Self> = VecDeque::new();
    let mut name : Option<RoomName> = None;
    let mut doors : HashMap<Direction, Option<RoomName>> = HashMap::new();
    let mut items : HashSet<ItemName> = HashSet::new();
    let mut description : Option<String> = None;
    
    let string : String = output.into_iter().map(|c| *c as u8 as char).collect();
    //println!("{}", string);
    let mut state = 0;
    for line in string.lines() {
    
      if line.is_empty() { 
        state = 0; 
      } else if state == 0 {
        if line == "Doors here lead:" { 
          state = 1;
        } else if line == "Items here:" { 
          state = 2;
        } else if line == "Command?" {
          
        } else if line.starts_with("==") {
          if !name.is_none() {
            ret.push_back(Room {
              name: name.unwrap(),
              from: None,
              description: description.unwrap(),
              doors,
              items,
            });
          }
          name = Some(line.to_string());
          doors = HashMap::new();
          items = HashSet::new();
          description = None;          
        } else {
          if description.is_none() {
            description = Some(line.to_string());
          }
        }
      } else if state == 1 {
        doors.insert(Direction::from_str(&line[2..]), None);
      } else if state == 2 {
        items.insert(line[2..].to_string());
      }
    }
    if doors.is_empty() {
      println!("{}", string);
      panic!("no direction available !");
    }
    ret.push_back(Room {
      name: name.unwrap(),
      from: None,
      description: description.unwrap(),
      doors,
      items,
    });
    return ret;
  }
}

fn explore_rec(mut state: &mut State, mut rooms: &mut HashMap<RoomName, Room>, parent_name: &RoomName, direction: Direction) -> RoomName {
  let mut input : VecDeque<i64> = VecDeque::new();
  for c in direction.as_str().chars() {
    input.push_back(c as u8 as i64);
  }
  input.push_back(10);
  let mut output = state.process(&mut input);
  let mut new_rooms = Room::new_from_output(&mut output);
  let mut room = new_rooms.pop_front().unwrap();
  let room_name = room.name.to_string();
  let mut doors : HashMap<Direction, Option<RoomName>> = HashMap::new();
  for door in room.doors.keys() {
    if door.invert() == direction { continue; }
    doors.insert(*door, Some(explore_rec(&mut state, &mut rooms, &room_name, *door)));
  }
  room.doors = doors;
  room.from = Some((direction.invert(), parent_name.to_string()));
  rooms.insert(room.name.to_string(), room);
  if new_rooms.is_empty() { // we don't need to go back if we have been ejected from the security room
    for c in direction.invert().as_str().chars() {
      input.push_back(c as u8 as i64);
    }
    input.push_back(10);
    state.process(&mut input);
  }
  return room_name;
}

fn explore(code: &Vec<i64>) -> HashMap<RoomName, Room> {
  let mut rooms : HashMap<RoomName, Room> = HashMap::new();
  let mut state = State::new_from_vector(&code);
  let mut input : VecDeque<i64> = VecDeque::new();
  let mut output = state.process(&mut input);
  let mut room = Room::new_from_output(&mut output).pop_front().unwrap();
  let room_name = room.name.to_string();
  let mut doors : HashMap<Direction, Option<RoomName>> = HashMap::new();
  for door in room.doors.keys() {
    doors.insert(*door, Some(explore_rec(&mut state, &mut rooms, &room_name, *door)));
  }
  room.doors = doors;
  rooms.insert(room_name.to_string(), room);
  return rooms;
}

fn explore_manual(filename: impl AsRef<std::path::Path>) -> () {
  let code = parse(filename);
  let mut state = State::new_from_vector(&code);
  let mut input : VecDeque<i64> = VecDeque::new();
  loop {
    let output = state.process(&mut input);
    let (grid, _) = output_to_grid(&output);
    grid.print();
    let mut line = String::new();
    io::stdin().read_line(&mut line);
    for c in line.chars() {
      input.push_back(c as u8 as i64);
    }
  }
}

fn send_command<T: Deref<Target = str>>(mut state: &mut State, command: T) -> VecDeque<i64>{
  let mut input : VecDeque<i64> = VecDeque::new();
  for c in command.chars() {
    input.push_back(c as u8 as i64);
  };
  input.push_back(10);
  let string : String = input.iter().map(|c| *c as u8 as char).collect();
 // println!("command:{}", string);
  let output = state.process(&mut input);
 // let string : String = output.iter().map(|c| *c as u8 as char).collect();
 // println!("output:{}", string);
  
  return output;
}

fn parse_security(output : VecDeque<i64>) -> (bool, String) {
  let string : String = output.into_iter().map(|c| c as u8 as char).collect();
  let mut desc : Vec<String> = Vec::new();
  let mut already_one = false;
  let mut state = 0;
  for line in string.lines() {
    if line.is_empty() { continue; }
    if line.starts_with("==") {
      if already_one { 
        return (false, desc.join("\n")); 
      }
      already_one = true;
    }
    if line == "== Pressure-Sensitive Floor ==" {
      state = 1;
      continue;
    }
    if state == 1 && line == "Doors here lead:" {
      state = 2;
      continue;
    }
    if state == 2 && !line.starts_with("- ") {
      desc.push(line.to_string());
    }
  }
  //println!("{}", string);
  return (true, desc.join("\n"));
}

fn go_to(mut state: &mut State, rooms: &HashMap<RoomName, Room>, room_name: &RoomName) -> (bool, String) {
  let mut directions : Vec<Direction> = Vec::new();
  let mut cur = room_name.to_string();
  loop {
    let room = rooms.get(&cur).unwrap();
    if room.from.is_none() { break; }
    let (dir, new_cur) = room.from.as_ref().unwrap();
    cur = new_cur.to_string();
    directions.push(dir.invert());
  }
  let mut output : Option<VecDeque<i64>> = None;
  while !directions.is_empty() {
    output = Some(send_command(state, directions.pop().unwrap().as_str()));
  }
  if output.is_none() { return (false, String::from("unexpected")); }
  return parse_security(output.unwrap());
}

fn go_back(state: &mut State, rooms: &HashMap<RoomName, Room>, room_name: &RoomName){
  let mut cur = room_name.to_string();
  loop {
    let room = rooms.get(&cur).unwrap();
    if room.from.is_none() { break; }
    let (dir, new_cur) = room.from.as_ref().unwrap();
    cur = new_cur.to_string();
    send_command(state, dir.as_str());
  }
}

fn take(state: &mut State, rooms: &mut HashMap<RoomName, Room>, items: &HashMap<ItemName, RoomName>, item: &ItemName){
  let room_name = items.get(item).unwrap();
  println!("Go to");
  go_to(state, rooms, room_name);
  println!("Take cmd");
  let output = send_command(state, format!("take {}", item));
  let string : String = output.iter().map(|c| *c as u8 as char).collect();
  let mut desc = String::new();
  for line in string.lines() {
    if !line.is_empty() && line != "Command?" { 
      desc = line.to_string();
    }
  }
  println!("response from take: {}", desc);
  println!("Go back");
  go_back(state, rooms, room_name);
}

fn drop(state: &mut State, rooms: &mut HashMap<RoomName, Room>, items: &HashMap<ItemName, RoomName>, item: &ItemName){
  let room_name = items.get(item).unwrap();
  go_to(state, rooms, room_name);
  send_command(state, format!("drop {}", item));
  go_back(state, rooms, room_name);
}

fn test(state: &mut State, rooms: &mut HashMap<RoomName, Room>, items: &HashMap<ItemName, RoomName>, tested_items: &Vec<ItemName>){
  println!("Take");
  for item in tested_items {
    take(state, rooms, items, &item);
  }
  println!("Got to security");
  let (success, desc) = go_to(state, rooms, &String::from("== Pressure-Sensitive Floor =="));
  if success {
    println!("success: {}", desc);
    return;
  } else {
    println!("fail: {}", desc);
  }
  println!("Go back to entrance");
  go_back(state, rooms, &String::from("== Pressure-Sensitive Floor =="));
  println!("Drop items");
  for item in tested_items {
    drop(state, rooms, items, &item);
  }
}

fn extract_items(rooms : &HashMap<RoomName, Room>) -> Vec<ItemName> {
  return rooms.values().flat_map(|r| r.items.iter().map(|s| s.to_string())).collect();
}

fn extract_roomname(rooms : &HashMap<RoomName, Room>) -> Vec<RoomName> {
  return rooms.keys().map(|s| s.to_string()).collect();
}

fn explore_auto(filename: impl AsRef<std::path::Path>){
  let code = parse(filename);
  let rooms = explore(&code);
  println!("Items:\n{}", extract_items(&rooms).join("\n"));
  println!("");
  println!("Rooms:\n{}", extract_roomname(&rooms).join("\n"));
}

fn q1(filename: impl AsRef<std::path::Path>){
  let code = parse(filename);
  let mut rooms = explore(&code);
  println!("Items:\n{}", extract_items(&rooms).join("\n"));
  println!("");
  println!("Rooms:\n{}", extract_roomname(&rooms).join("\n"));
  let mut state = State::new_from_vector(&code);
  let mut items : HashMap<ItemName, RoomName> = HashMap::new();
  for room in rooms.values() {
    for item in &room.items {
      items.insert(item.to_string(), room.name.to_string());
    }
  }
  test(&mut state, &mut rooms, &mut items, &vec![
    String::from("jam"),
    //String::from("dark matter"),
    //String::from("planetoid"),
    String::from("spool of cat6"),
    String::from("fuel cell"),
    //String::from("wreath"),
    String::from("sand")
  ]);
}
// cannot take molten lava => melt
// cannot take infinite loop => do an infinite loop...
// cannot take escape pod => ejected
// cannot take giant electromagnet => stuck
// cannot take photons => completely dark!

// jam => not heavy enough
// dark matter => not heavy enough
// planetoid => not heavy enough
// spool of cat6 => not heavy enough
// fuel cell => not heavy enough
// wreath => not heavy enough
// sand => not heavy enough

// jam, dark matter, planetoid => not heavy enough
// jam, dark matter, planetoid, spool of cat6, fuel cell, wreath => not heavy enough
// jam, dark matter, planetoid, spool of cat6, sand => not heavy enough

// jam, dark matter, spool of cat6, fuel cell, sand => too heavy
// jam, dark matter, planetoid, spool of cat6, fuel cell, sand => too heavy
// jam, dark matter, planetoid, spool of cat6, fuel cell, wreath, sand => too heavy


// coin => too heavy

fn main() {
  q1("data.txt");
}
