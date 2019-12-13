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

struct Instruction {
    num_params: u32,
    
}

#[derive(Clone, Debug)]
struct IntcodeComputer {
    regs: Vec<i32>,
    pc: usize,
    input_queue: Vec<i32>,
    output_queue: Vec<i32>,
}
impl IntcodeComputer {
    fn input(&mut self, val: i32) {
        self.input_queue.push(val);
    }
    fn output(&mut self, val: i32) {
        self.output_queue.push(val);
    }
    fn run(&mut self, noun: Option<i32>, verb: Option<i32>) {
        if let Some(n) = noun {
            self.regs[1] = n;
        }
        if let Some(v) = verb {
            self.regs[2] = v;
        }
    }
}


fn main() {
    let memory = match stdin_to_array() {
        Ok(x) => x,
        Err(e) => {
            println!("Error getting input: {}", e);
            return;
        }
    };
    let target = 19690720;

    // Noun/verb for the program
    for noun in 0..99 {
        for verb in 0..99 {
            let ret_val: i32 = run_program(&memory, noun, verb);
            if ret_val == target {
                println!("Found target. Noun: {}, Verb: {}", noun, verb);
                return;
            }
        }
    }
}

fn run_program(memory: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    // Copy and adjust mem for this run
    let mut regs: Vec<i32> = memory.to_vec();
    regs[1] = noun;
    regs[2] = verb;
    let mut pc: usize = 0;
    loop {
        if pc >= regs.len() {
            println!("Error, reached end of memory without terminating.");
            return -1;
        }
        match regs[pc] {
        1 => {
            let left_index: usize = regs[pc+1] as usize;
            let right_index: usize = regs[pc+2] as usize;
            let target_index: usize = regs[pc+3] as usize;
            regs[target_index] = regs[left_index] + regs[right_index];
            pc += 4;
        }
        2 => {
            let left_index: usize = regs[pc+1] as usize;
            let right_index: usize = regs[pc+2] as usize;
            let target_index: usize = regs[pc+3] as usize;
            regs[target_index] = regs[left_index] * regs[right_index];
            pc += 4;
        }
        99 => {
//            pc += 1;
            return regs[0];
        }
        x => {
            println!("Error, unexpected instruction at location {}: {}", pc, x);
            println!("{:?}", regs);
            return -1;
        }
    }
}
}
