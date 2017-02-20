use std::collections::HashMap;

#[derive(Debug)]
struct Frame {
    num_bindings: usize,
    bindings: Vec<Option<Value>>,
}
#[derive(Debug, Copy, Clone)]
enum Value {
    Int(i32),
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Literal(Value),
    Add,
    Sub,
    Mul,
    IntDiv,
    Output,
}

#[derive(Debug)]
enum Error {
    StackUnderflow,
    InstructionOverflow,
    InvalidTypes,
    NoMain,
}

macro_rules! math_op {
    ($stack: expr => $func: expr) => (
        {
            let b = ($stack).pop().ok_or(Error::StackUnderflow)?;
            let a = ($stack).pop().ok_or(Error::StackUnderflow)?;
            match (a, b) {
                (Value::Int(n1), Value::Int(n2)) => {
                    ($stack).push(Value::Int($func(n1, n2)))
                },
                //_ => return Err(Error::InvalidTypes),
            }
        }
    )
}

type CallStack = Vec<Frame>;
type DataStack = Vec<Value>;

struct Program {
    instructions: Vec<Instruction>,
    frame_info: HashMap<String, FrameInfo>,
}

struct FrameInfo {
    num_bindings: usize,
    position: usize,
}

fn run(program: Program) -> Result<(), Error> {
    let mut ipstack: Vec<usize>  = Vec::new();
    let mut callstack: CallStack = Vec::new();
    let mut datastack: DataStack = Vec::new();
    let program_len = program.instructions.len();
    let mut current_frame = program.frame_info.get("main").ok_or(Error::NoMain)?;
    let mut ip = current_frame.position;
    while ip < program_len {
        let instruction = program.instructions.get(ip).ok_or(Error::InstructionOverflow)?;
        match *instruction {
            Instruction::Literal(n) => datastack.push(n),
            Instruction::Add => math_op!(datastack => |a, b| a + b),
            Instruction::Sub => math_op!(datastack => |a, b| a - b),
            Instruction::Mul => math_op!(datastack => |a, b| a * b),
            Instruction::IntDiv => math_op!(datastack => |a, b| (a / b) as i32),
            Instruction::Output => {
                let val = datastack.pop().ok_or(Error::StackUnderflow)?;
                println!("{:?}", val);
            }
        };
        ip += 1;
    }
    Ok(())
}

fn main() {
    let mut frame_info = HashMap::new();
    frame_info.insert(String::from("main"), FrameInfo {
        num_bindings: 0,
        position: 0,
    });
    let prog = Program {
            instructions: vec![
                Instruction::Literal(Value::Int(1)),
                Instruction::Literal(Value::Int(1)),
                Instruction::Add,
                Instruction::Literal(Value::Int(2)),
                Instruction::Mul,
                Instruction::Literal(Value::Int(1)),
                Instruction::Sub,
                Instruction::Literal(Value::Int(2)),
                Instruction::IntDiv,
                Instruction::Output,
            ],
            frame_info: frame_info,
    };
    run(prog).unwrap();
}
