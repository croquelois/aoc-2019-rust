
fn parse_output(output : VecDeque<i64>) -> (HashSet<String>, Vec<String>, String, bool) {
  let mut doors : HashSet<String> = HashSet::new();
  let mut items : Vec<String> = Vec::new();
  let mut description : Vec<String> = Vec::new();
  let mut security = false;
  let string : String = output.into_iter().map(|c| c as u8 as char).collect();
  println!("{}", string);
  let mut state = 0;
  for line in string.lines() {
    if line.is_empty() { 
      state = 0; 
    } else if state == 0 {
      if line == "Doors here lead:" { 
        state = 1;
        doors = HashSet::new();
      } else if line == "Items here:" { 
        state = 2;
        items = Vec::new();
      } else if line == "Command?" {
        
      } else if line == "== Pressure-Sensitive Floor ==" {
        security = true;
      } else {
        description.push(line.to_string());
      }
    } else if state == 1 {
      doors.insert(line[2..].to_string());
    } else if state == 2 {
      items.push(line[2..].to_string());
    }
  }
  if doors.is_empty() {
    println!("{}", string);
    panic!("no direction available !");
  }
  return (doors, items, description.join(" => "), security);
}

fn pos_x2(pos : &Point) -> Point { (pos.0*2, pos.1*2) }

fn move_direction(pos: &Point, dir: &str) -> Point {
  match dir {
    "north" => (pos.0,pos.1-1),
    "south" => (pos.0,pos.1+1),
    "east" => (pos.0+1,pos.1),
    "west" => (pos.0-1,pos.1),
    _ => panic!("unexpected direction: {}", dir)
  }
}

fn explore(code: &Vec<i64>, nb_step: usize) -> Grid {
  let mut grid = Grid::new();
  let mut state = State::new_from_vector(&code);
  let mut input : VecDeque<i64> = VecDeque::new();
  let mut pos : Point = (0,0);
  let mut old_pos : Point = (0,0);
  let mut seen : HashSet<Point> = HashSet::new();
  let mut step = 0;
  loop {
	  let (doors, items, desc, security) = parse_output(state.process(&mut input));
    grid.set(pos_x2(&old_pos), '.');
    if security {
      grid.set(pos_x2(&pos),'S');
      pos = old_pos;
    } else {
      grid.set(pos_x2(&pos),'.');
    }
    grid.set(pos_x2(&pos), '@');
    if seen.insert(pos) {      
    }
    println!("{:?} {} items:{:?}",pos,desc,items);
    for dir in &DIRECTION {
      let new_pos_x2 = move_direction(&pos_x2(&pos), dir);
      let mut c = grid.get(&new_pos_x2);
      let has_door = doors.contains(&dir.to_string());
      if c == ' ' {
        c = if has_door { '.' } else { '#' };
      } else if c == '#' && has_door {
        c = match *dir {
          "north" => '^',
          "south" => 'v',
          "east" => '<',
          "west" => '>',
          _ => panic!("unexpected direction: {}", dir)
        }
      } else if c == '.' && !has_door {
        c = match *dir {
          "north" => 'v',
          "south" => '^',
          "east" => '>',
          "west" => '<',
          _ => panic!("unexpected direction: {}", dir)
        }
      
      }
      grid.set(new_pos_x2, c);
    }
    
    grid.print();
    
    step = step + 1;
    let mut dir = String::new();
    if nb_step > 0 {
      dir = random_sample(doors.into_iter()).unwrap();
      if step == nb_step { return grid; }
      println!("move: {}", dir);
    } else {
      loop {
        io::stdin().read_line(&mut dir);
        dir = dir.lines().next().unwrap().to_string();
        //println!("doors: {:?} input:{}", doors, dir);
        if doors.contains(&dir) { break; }
        dir = String::new();
      }
    }
    for c in dir.chars() {
      input.push_back(c as u8 as i64);
    }
    input.push_back(10);
    old_pos = pos;
    pos = move_direction(&pos, dir.as_str());
  }
}