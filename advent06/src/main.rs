use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Node {
  name: String,
  parent: String,
  childs: HashSet<String>,
}

fn parse_line(line: &str) -> (&str, &str) {
  let arr : Vec<&str> = line.split(")").collect();
  return (arr[0], arr[1]);
}

fn create_new_node(name: &str) -> Node {
  return Node {
    name: name.to_string(),
    parent: String::new(),
    childs: HashSet::new()
  }
}

fn populate_tree(tree : &mut HashMap<String, Node>, edge : &(&str, &str)) {
  tree.entry(edge.0.to_string()).or_insert(create_new_node(edge.0)).childs.insert(edge.1.to_string());
  tree.entry(edge.1.to_string()).or_insert(create_new_node(edge.1)).parent = edge.0.to_string();
}

fn parse(filename: impl AsRef<std::path::Path>) -> HashMap<String, Node> {
  let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let edges : Vec<(&str, &str)> = data.lines().map(parse_line).collect();
  let mut tree : HashMap<String, Node> = HashMap::new();
  for edge in edges {
    populate_tree(&mut tree, &edge);
  }
  return tree;
}

fn count_orbit(tree : &HashMap<String, Node>, root: &String, depth: usize) -> usize {
  let mut sum : usize = depth;
  let childs : &HashSet<String> = &tree.get(root).expect("unknow root").childs;
  for child in childs {
    sum = sum + count_orbit(tree, &child, depth + 1)
  }
  return sum;
}

fn q1(filename: impl AsRef<std::path::Path>) -> usize {
  return count_orbit(&parse(filename), &"COM".to_string(), 0);
}

fn extract_path(tree : &HashMap<String, Node>, who: &str) -> HashSet<String> {
  let mut ret : HashSet<String> = HashSet::new();
  let mut cur = &who.to_string();
  ret.insert(cur.to_string());
  while cur != "COM" {
    cur = &tree.get(cur).expect("broken tree").parent;
    ret.insert(cur.to_string());
  }
  return ret;
}

fn q2(filename: impl AsRef<std::path::Path>) -> usize {
  let tree = parse(filename);
  let hash_you = extract_path(&tree, "YOU");
  let hash_santa = extract_path(&tree, "SAN");
  let intersection : Vec<_> = hash_you.intersection(&hash_santa).collect();
  return hash_you.len() + hash_santa.len() - 2*intersection.len() + 1 - 3;
}

fn main() {
  println!("Question1: {}", q1("data.txt"));
  println!("Question2: {}", q2("data.txt"));
}

#[test]
fn test_examples1() {
  assert_eq!(q1("test1.txt"), 42);
}

#[test]
fn test_examples2() {
  assert_eq!(q2("test2.txt"), 4);
}