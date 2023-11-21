use crate::bf::cell::Cell;
use crate::bf::prog::Program;

pub mod cell;
pub mod prog;

/// Error handling for bf
#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for crate::bf::Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type AppResult = Result<(), self::Error>;

impl std::error::Error for crate::bf::Error{
    fn description(&self) -> &str {
        &self.0
    }
}

/// Bf struct
/// main brainf*ck system is based on it.
pub struct Bf {
    memory: Cell,
    program: Program,
}

/// Brainf*ck default memory size
pub const BF_DEFAULT_CELL: usize = 0x7530; // 30,000 byte

impl Bf{
    /// creates a new program
    pub fn new(input: String)->Bf {
        Bf {
            program: Program::new(input),
            memory: Cell::new(),
        }
    }

    pub fn interpret(&mut self){
        if self.memory.ptr() >= self.memory.len(){
            return;
        }
        match self.program.execute(){
            Ok(()) => {
                std::process::exit(0);
            },
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

