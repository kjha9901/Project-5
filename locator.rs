use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    
    
    fn enqueue(&mut self, ele: T) -> () {
      self.push(ele);
      let mut ele_loc = self.len()-1;
      
      if self.len() > 1 {
        let mut parent_loc = (ele_loc - 1) / 2;
        
        while self[ele_loc] < self[parent_loc] && ele_loc != parent_loc {
          self.swap(ele_loc, parent_loc);
          
          let mut temp = ele_loc;
          ele_loc = parent_loc;
          parent_loc = (temp - 1) / 2;
          
          if parent_loc < 0 {
            parent_loc = 0;
          }
          
        }
        
      }
      
    }

    
    
    fn dequeue(&mut self) -> Option<T> {
      if self.len() == 0 {
        return None;
      }
      
      let root = self.remove(0);
      let mut ele_loc = 0;
      let mut child_loc_left = 2*ele_loc+1;
      let mut child_loc_right = 2*ele_loc+2;
      let length = self.len();
      
      self.swap(0,length-1);
      self.pop();
      
      if self.len() > 1 {
        while self[ele_loc] > self[child_loc_left] || self[ele_loc] > self[child_loc_right]{
          if self[ele_loc] > self[child_loc_left] {
            self.swap(ele_loc, child_loc_left);
            let mut temp = ele_loc;
            ele_loc = child_loc_left;
            child_loc_left = (2*temp)+1;
            child_loc_right = (2*temp)+2;
            if (child_loc_left >= self.len() || child_loc_right >= self.len()) || (child_loc_left == ele_loc || child_loc_right == ele_loc) {
              break;
            }
          }
          else {
            self.swap(ele_loc, child_loc_right);
            let mut temp = ele_loc;
            ele_loc = child_loc_right;
            child_loc_left = 2*temp+1;
            child_loc_right = 2*temp+2;
            if (child_loc_left >= self.len() || child_loc_right >= self.len()) || (child_loc_left == ele_loc || child_loc_right == ele_loc) {
              break;
            }
          }
        }
      }
      
      return Some(root);
    }


    fn peek(&self) -> Option<&T> {
      if self.len() == 0 {
        return None;
      }
      return self.first();
    }
}


/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    if p1.0 < p2.0 {
      if p1.1 < p2.1 {
        println!("\ndistance1 = {:?}", (p2.0-p1.0)+(p2.1-p1.1));
        return (p2.0-p1.0)+(p2.1-p1.1);
      }
      else {
        println!("\ndistance2 = {:?}",(p2.0-p1.0)+(p1.1-p2.1));
        return (p2.0-p1.0)+(p1.1-p2.1);
      }
    }
    else {
      if p1.1 < p2.1 {
        println!("\ndistance3 = {:?}",(p1.0-p2.0)+(p2.1-p1.1));
        return (p1.0-p2.0)+(p2.1-p1.1);
      }
      else {
        println!("\ndistance4 = {:?}",(p1.0-p2.0)+(p1.1-p2.1));
        return (p1.0-p2.0)+(p1.1-p2.1);
      }
    }
}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.    
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {
  
  let mut stark = ("",0,0);
  
  for (k1, v1) in allies.iter() {
    if k1 == &"Stark" {
      stark = (k1,v1.0,v1.1);
    }
  }
  
  let mut dist = 0;
  let mut dist2 = 0;
  let mut min = 10000;
  let mut enemy = ("",0,0);
  
  for (k2,v2) in enemies.iter() {
    dist = distance((stark.1,stark.2),(v2.0,v2.1));
    
    if dist < min {
      
      min = dist;
      let mut starks_enemy = true;
      
      for (k3,v3) in allies.iter() {
        dist2 = distance((v3.0,v3.1),(v2.0,v2.1));
        
        if dist2 < dist && k3 != &"Stark" {
          starks_enemy = false;
        }
      }
      
      if starks_enemy == true {
        enemy = (k2,v2.0,v2.1);
      }
      
    }
  }
  
  return enemy;
}








