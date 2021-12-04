#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
  pub fn as_str (&self) -> String {
    
    if let Command::Power(true, num) = self {
      return format!("Power increased by {}%",num);
    }
    else if let Command::Power(false, num) = self {
      return format!("Power decreased by {}%",num);
    }
    else if let Command::Missiles(true, num) = self {
      return format!("Missiles increased by {}",num);
    }
    else if let Command::Missiles(false, num) = self {
      return format!("Missiles decreased by {}",num);
    }
    else if let Command::Shield(true) = self {
      return "Shield turned on".to_string();
    }
    else if let Command::Shield(false) = self {
      return "Shield turned off".to_string();
    }
    else if let Command::Try = self {
      return "Call attempt failed".to_string();
    }
    else {
      return "Not a command".to_string();
    }
  }
}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {

  let mut split: Vec<&str> = s.split(" ").collect();
  let mut num = 0;
  
  if s == "try calling Miss Potts" {
    return Command::Try;
  }
  else if split.len() == 3 && split[0] == "power" {
    if split[1] == "inc" {
      num = split[2].parse().unwrap();
      return Command::Power(true,num);
    }
    else if split[1] == "dec" {
      num = split[2].parse().unwrap();
      return Command::Power(false,num);
    }
  }
  else if split.len() == 3 && split[0] == "add" {
    num = split[1].parse().unwrap();
    return Command::Missiles(true,num);
  }
  else if split.len() == 3 && split[0] == "fire" {
    num = split[1].parse().unwrap();
    return Command::Missiles(false,num);
  }
  else if split.len() == 2 && split[0] == "shield" {
    if split[1] == "on" {
      return Command::Shield(true);
    }
    else if split[1] == "off" {
      return Command::Shield(false);
    }
  }
  else {
    return Command::Invalid;
  }
  return Command::Invalid;
}









