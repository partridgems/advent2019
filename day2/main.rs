use std::io::{self, Read};
use std::error::Error;

fn stdin_to_array() -> Result<Vec<i32>, Box<dyn Error>> {
  let mut parsed_vec: Vec<i32> = Vec::new();
  let mut input = String::new();
  io::stdin().lock().read_to_string(&mut input)?;
  for val in input.split(',') {
    parsed_vec.push(val.trim().parse()?);
  }

  return Ok(parsed_vec);
}

fn main() {
  let mut regs = match stdin_to_array() {
    Ok(x) => x,
    Err(e) => {
      println!("Error getting input: {}", e);
      return;
    }
  };

  for pc in (0..regs.len()).step_by(4) {
    match regs[pc] {
      1 => {
        regs[pc+3] = regs[pc+1] + regs[pc+2];
      }
      2 => {
        regs[pc+3] = regs[pc+1] * regs[pc+2];
      }
      99 => {
        println!("Program completed: {}", regs[0]);
      }
      x => {
        println!("Error, unexpected instruction at location {}: {}", pc, x);
        println!("{:?}", regs);
        return;
      }
    }
  }
  println!("Error, reached end of program with no termination");
}
