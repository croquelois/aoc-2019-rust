
fn check1(n: i32) -> bool {
  let mut acc = n;
  let mut prev = acc % 10;
  let mut cur;
  let mut dup = false;
  acc /= 10;
  while acc > 0 {
    cur = acc % 10;
    acc /= 10;
    if cur > prev {
      return false;
    }
    if cur == prev {
      dup = true;
    }
    prev = cur;
  }
  return dup;
}

fn check2(n: i32) -> bool {
  let mut acc = n;
  let mut prev = acc % 10;
  let mut cur;
  let mut dup_cnt = 1;
  let mut dup2 = false;
  acc /= 10;
  while acc > 0 {
    cur = acc % 10;
    acc /= 10;
    if cur > prev {
      return false;
    }
    if cur == prev {
      dup_cnt += 1;
    }
    else
    {
      if dup_cnt == 2 {
        dup2 = true;
      }
      dup_cnt = 1;
    }
    prev = cur;
  }
  if dup_cnt == 2 {
    dup2 = true;
  }
  return dup2;
}


fn q1(s: i32, e: i32) -> i32 {
  let mut count : i32 = 0;
  for i in s..e {
    if check1(i) {
     count += 1;
    }
  }
  return count;
}

fn q2(s: i32, e: i32) -> i32 {
  let mut count : i32 = 0;
  for i in s..e {
    if check2(i) {
     count += 1;
    }
  }
  return count;
}

fn main() {
    println!("Question1: {}", q1(387638,919123));
    println!("Question2: {}", q2(387638,919123));
}

#[test]
fn test1() {
  assert_eq!(check1(111111), true);
  assert_eq!(check1(223450), false);
  assert_eq!(check1(123789), false);
}

#[test]
fn test2() {
  assert_eq!(check2(112233), true);
  assert_eq!(check2(123444), false);
  assert_eq!(check2(111122), true);
}
