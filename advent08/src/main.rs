use std::fs;

#[derive(Debug)]
struct Layer {
  width: usize,
  height: usize,
  data: Vec<i32>
}

impl Layer {
  fn transparent(width: usize, height: usize) -> Layer {
    let mut data = Vec::new();
    for _ in 0..(width*height)
    {
      data.push(2);
    }
    return Layer {
      width: width,
      height: height,
      data: data
    };
  }
  fn count_digit(&self, digit: i32) -> i32 {
    return self.data.iter().fold(0, |n,d| if *d==digit { n+1 } else { n });
  }
  fn merge(&mut self, layer: &Layer) -> () {
    for i in 0..self.data.len()
    {
      if self.data[i] == 2
      {
        self.data[i] = layer.data[i];
      }
    }
  }
  fn print(&self) -> () {
    for y in 0..self.height
    {
      let mut line = String::new();
      for x in 0..self.width
      {
        let color = self.data[x+y*self.width];
        if color == 1
        {
          line = format!("{}{}", line, "O");
        }
        else
        {
          line = format!("{}{}", line, " ");
        }
      }
      println!("{}", line);
    }
  }
}

fn parse_layers(data: &Vec<i32>, width: usize, height: usize) -> Vec<Layer>
{
  let mut ret = Vec::new();
  let data_length = width*height;
  let n = data.len() / data_length;
  for i in 0..n 
  {
    ret.push(Layer {
      width: width,
      height: height,
      data: data[i*data_length..(i+1)*data_length].to_vec()
    });
  }
  return ret;
}

fn parse(filename: impl AsRef<std::path::Path>) -> Vec<i32> {
  return fs::read_to_string(filename).expect("Something went wrong reading the file")
    .chars().map(|s| s.to_string().parse::<i32>().unwrap()).collect();
}

fn q1(filename: impl AsRef<std::path::Path>) -> i32 {
  let data = parse(filename);
  let layers = parse_layers(&data,25,6);
  let mut min_0 = 999999;
  let mut check = 0;
  for layer in layers
  {
    let nb_0 = layer.count_digit(0);
    if nb_0 < min_0
    {
      min_0 = nb_0;
      check = layer.count_digit(1) * layer.count_digit(2);
    }
  }
  return check;
}

fn q2(filename: impl AsRef<std::path::Path>) -> () {
  let data = parse(filename);
  let layers = parse_layers(&data,25,6);
  let mut img = Layer::transparent(25,6);
  for layer in layers
  {
    img.merge(&layer);
  }
  img.print();
}

fn main() {
  println!("{:?}", q1("data.txt"));
  q2("data.txt");
}
