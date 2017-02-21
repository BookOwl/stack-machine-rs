use std::collections::HashMap;


#[derive(Debug)]
struct Frame {
    num_bindings: usize,
    bindings: Vec<Option<Value>>,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Value {
    Int(i32),
    Bool(bool),
}

#[derive(Debug)]
enum Instruction {
    Literal(Value),
    Add,
    Sub,
    Mul,
    IntDiv,
    Output,
    Equal,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Exit,
    Unreachable,
    Jump(String)
}

#[derive(Debug)]
enum Error {
    StackUnderflow,
    InstructionOverflow,
    InvalidTypes,
    NoMain,
    MissingLabel,
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

// Macros to assist in writing operations.

/// A helper macro for op! that pops as many items from the stack as there are patterns
macro_rules! pops {
    // If there is just one patten pop once
    ($stack:ident; $head:pat) => ($stack.pop().ok_or(Error::StackUnderflow)?);
    // Otherwise pop once for the head pattern and also pop for each of rest.
    ($stack:ident; $head:pat, $($tail:pat),*) => (($stack.pop().ok_or(Error::StackUnderflow)?,
                                            pops!($stack; $($tail),*)));
}

/// A helper macro for op! that reverses the patterns passed to it so a, b becomes b, a
macro_rules! reverse_pats {
    // If there is just one pattern, the reverse is just that pattern
    ($head:pat) => ($head);
    // If there is more than one patern the reverse is
    // the first pattern behind the reverse of the rest of the patterns
    ($head:pat, $($tail:pat),*) => ((
        reverse_pats!($($tail),*), $head
    ));
}

/// A macro to assist in defining operators.
/// You pass in the datastack, the patterns for the operator,
/// and an expression that computes the result of the operator.
///
/// Example:
/// ```
///Instruction::Add => op!(datastack; Value::Int(a), Value::Int(b) => a + b)
/// ```
macro_rules! op {
    ($stack: ident; $( $p:pat ),+ => $exp:expr) => (
        {
            // Pattern match on same number of pops as there are patterns
            match pops!($stack; $($p),+) {
                // Reverse the patterns in the match arm so that you can write
                // the operators like a, b => a + b where b is at the TOS
                reverse_pats!($($p),+) if true => {
                    //     ^^^^^^^ This is needed to allow non-exhausitve patterns.
                    // https://github.com/rust-lang/rust/issues/4112#issuecomment-135410438
                    ($stack).push($exp)
                },
                // Invalid types, so return an Error
                _ => return Err(Error::InvalidTypes)
            }
        }
    )
}

fn run(program: Program) -> Result<(), Error> {
    let mut ipstack: Vec<usize>  = Vec::new();
    let mut callstack: CallStack = Vec::new();
    let mut datastack: DataStack = Vec::new();
    let program_len = program.instructions.len();
    let mut current_frame = program.frame_info.get("main").ok_or(Error::NoMain)?;
    let mut ip = current_frame.position;
    let mut jumped = false;
    while ip < program_len {
        let instruction = program.instructions.get(ip).ok_or(Error::InstructionOverflow)?;
        match *instruction {
            Instruction::Literal(n) => datastack.push(n),
            Instruction::Add => op!(datastack; Value::Int(a), Value::Int(b) => Value::Int(a+b)),
            Instruction::Sub => op!(datastack; Value::Int(a), Value::Int(b) => Value::Int(a-b)),
            Instruction::Mul => op!(datastack; Value::Int(a), Value::Int(b) => Value::Int(a*b)),
            Instruction::IntDiv => op!(datastack; Value::Int(a), Value::Int(b) => Value::Int((a/b) as i32)),
            Instruction::Output => {
                println!("{:?}", datastack.pop().ok_or(Error::StackUnderflow)?);
            },
            Instruction::Equal => op!(datastack; a, b => Value::Bool(a == b)),
            Instruction::Less => op!(datastack; a, b => Value::Bool(a < b)),
            Instruction::Greater => op!(datastack; a, b => Value::Bool(a > b)),
            Instruction::LessEq => op!(datastack; a, b => Value::Bool(a <= b)),
            Instruction::GreaterEq => op!(datastack; a, b => Value::Bool(a >= b)),
            Instruction::Exit => break,
            Instruction::Unreachable => panic!("Unreachable instruction executed! D:"),
            Instruction::Jump(ref label_name) => {
                let label = program.frame_info.get(label_name).ok_or(Error::MissingLabel)?;
                ip = label.position;
                jumped = true;
            },
        };
        if !jumped {
            ip += 1;
        } else {
            jumped = false;
        }
    }
    Ok(())
}

fn main() {
    let mut frame_info = HashMap::new();
    frame_info.insert(String::from("main"), FrameInfo {
        num_bindings: 0,
        position: 0,
    });
    frame_info.insert(String::from("foo"), FrameInfo {
        num_bindings: 0,
        position: 5,
    });
    let prog = Program {
            instructions: vec![
                Instruction::Literal(Value::Int(3)),
                Instruction::Literal(Value::Int(2)),
                Instruction::Sub,
                Instruction::Jump(String::from("foo")),
                Instruction::Literal(Value::Int(5)),
                Instruction::Output,
                Instruction::Exit,
                Instruction::Unreachable,
            ],
            frame_info: frame_info,
    };
    run(prog).unwrap();
}
