use aocf::Aoc;

pub fn run() {
    let mut instructions: Vec<Instruction> = process_input(&read_input());
    println!("The final value of the accumulator was {}.", fix_instructions(&mut instructions));
}
fn clone_instructions(instructions:&Vec<Instruction>) -> Vec<Instruction> {
    let mut output:Vec<Instruction> = Vec::new();
    for instruction in instructions {
        output.push(Instruction{operation:instruction.operation.clone(), argument:instruction.argument, has_been_run: false});
    }
    output
}

fn display_input(instructions:Vec<Instruction>) {
    for instruction in instructions {
        let mut display = String::new();
        match instruction.operation {
            Operation::Nop => display += "No Operation",
            Operation::Acc => display += "Accumulate",
            Operation::Jmp => display += "Jump",
            _ => display += "Serious error!!!"
        }
        println!("{} | {}", display, instruction.argument);
    }
}

enum Operation {
    Nop,
    Acc,
    Jmp,
    Err
}

impl Operation {
    fn create(op_str:&str) ->Operation {
        match op_str {
            "nop" => return Operation::Nop,
            "acc" => return Operation::Acc,
            "jmp" => return Operation::Jmp,
            _ => {
                println!("Serious error");
                return Operation::Err;
                }
        }
    }

    fn display(&self) {
        match self {
            Operation::Nop => println!("Nop"),
            Operation::Acc => println!("Acc"),
            Operation::Jmp => println!("Jmp"),
            _ => {
                println!("Serious error");
                }
        }
    }

    fn clone(&self) -> Operation {
        match self {
            Operation::Nop => return Operation::Nop,
            Operation::Acc => return Operation::Acc,
            Operation::Jmp => return Operation::Jmp,
            _ => {
                return Operation::Err
                }
        }
    }
}

struct Instruction {
    operation:Operation,
    argument: i32,
    has_been_run: bool
}

impl Instruction {
    fn jump(&self, i:usize, length:usize) -> usize{
        let length = length as usize;
        let mut output =(i as i32 + self.argument) as usize;
        if output < 0 {output+=length}
        output as usize
    }

    fn accumulate(&self, accumulator:i32) -> i32 {
        accumulator + self.argument
    }
}

fn fix_instructions(instructions:&mut Vec<Instruction>) -> i32 {
    let mut accumulator:i32 = 0;
    let mut i:usize = 0;
    while accumulator == 0 && i<instructions.len() {
        let steps = &mut clone_instructions(instructions);
        match steps[i].operation {
            Operation::Nop => {
                steps[i].operation = Operation::Jmp;
                accumulator = process_instructions(steps);
            },
            Operation::Jmp => {
                steps[i].operation = Operation::Nop;
                accumulator = process_instructions(steps);

            },
            _ => {
            },
        }
        i+=1;
    }
    accumulator
}

fn process_instructions(instructions:&mut Vec<Instruction>) -> i32 {
    let mut accumulator:i32 = 0;
    let mut active = true;
    let mut i:usize =0;
    let instructions_length = instructions.len();
    while active && i < instructions_length {
        if !instructions[i].has_been_run {
            instructions[i].has_been_run = true;
            match instructions[i].operation {
                Operation::Jmp => i=instructions[i].jump(i, instructions_length),
                Operation::Acc => {
                                    accumulator= instructions[i].accumulate(accumulator);
                                    i+=1;
                                }
                    ,
                Operation::Nop => i+=1,
                Operation::Err => println!("Serious Error")
            }
        } else {
            active = false;
        }

    }
    if i<instructions_length {
        accumulator = 0;
    }    
    accumulator
}

fn read_input() -> String {
    let mut aoc = Aoc::new().year(Some(2020)).day(Some(8)).init().unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false);
    let mut output = String::new();
    if let Ok(i) = input {
        output = i.trim().to_string();
    }
    return output.trim().to_string();
}

fn process_input(input: &str) -> Vec<Instruction> {
    let mut instructions:Vec<Instruction> = Vec::new();
    let input = input.split("\n");
    for line in input {
        let instruction = line.split(" ").collect::<Vec<&str>>();
        instructions.push(Instruction{operation:Operation::create(instruction[0]), argument:instruction[1].parse::<i32>().unwrap(), has_been_run:false});
    }
    instructions
}
