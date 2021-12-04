/** Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
  let mut sum = 0;
  if n < 0 {
    return -1;
  }
  else{
    for i in 1..n+1 {
      sum += i;
    }
    return sum;
  }
}


/** Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
  let mut total = 0;
  for i in 0..ls.len() {
    if ls[i] >= s && ls[i] <= e {
      total += 1;
    }
  }
  return total;
}


/** Returns true if target is a subset of set, false otherwise
    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
  for i in 0..target.len() {
    if !set.contains(&target[i]) {
      return false;
    }
  }
  return true;
}


/** Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
  if ls.len() == 0 {
    return None;
  }
  let mut sum = 0.0;
  let mut size = 0.0;
  for i in 0..ls.len() {
    sum += ls[i];
    size += 1.0;
  }
  return Some(sum/size);
}


/** Converts a binary number to decimal, where each bit is stored in order in the array
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
  let mut sum = 0;
  for i in 0..ls.len() {
    if ls[i] == 1 {
      sum += 2_i32.pow((ls.len() as u32)-(i as u32)-1)
    }
  }
  return sum;
}


/** Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2
    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
  fn fac(f:u32) -> u32 {
    if f % 2 == 0 {
      return 2;
    }
    for i in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= f) {
      if f % i == 0 {
        return i;
      };
    }
    return f;
  }
  let mut lst:Vec<u32> = Vec::new();
  let mut currentnum = n;
  loop {
    let m = fac(currentnum);
      lst.push(m);
      if m == currentnum {
        break;
      } else {
        currentnum /= m
      };
  }
  return lst;
}


/** Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
  let mut final_lst:Vec<i32> = Vec::new();
  if lst == [] {
    return final_lst;
  }
  for i in 1..lst.len() {
    final_lst.push(lst[i]);
  }
  final_lst.push(lst[0]);
  return final_lst;
}


/** Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
  if target == "" {
    return true;
  }
  for i in 0..s.len() {
    if (i as i32) + (target.len() as i32) <= (s.len() as i32) {
      let a = i as usize;
      let b = (i as usize) + (target.len() as usize);
      let slic = &s[a..b];
      if slic == target {
        return true;
      }
    }
  }
  return false;
}


/** Takes a string and returns the first longest substring of consecutive equal characters
    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
  if s == "" {
    return None;
  }
  let mut count = 0;
  let mut s2 = "";
  for i in 0..s.len() {
    let mut current = 1;
    
    for j in i..s.len() {
      if s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap() {
        break;
      }
      current+=1;
    }
    
    if current > count {
      count = current;
      s2 = &s[(i as usize)..((i as usize)+(count as usize)-1)];
    }
  }
  return Some(s2);
}





