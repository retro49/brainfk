use crate::bf::Error;
use crate::bf::cell::Cell;

use crate::bf::AppResult;

pub struct Program {
    cell: Cell,
    input: String,
    cursor: usize,
}

/// Base program implementation
/// Input is processed here
impl Program {
    /// creates a new program instance.
    pub fn new(input: String) -> Self {
        Program { 
            input, 
            cursor: 0, 
            cell: Cell::new() 
        }
    }

    /// returns the cursor of the input and not the memory.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Increases the input cursor by one.
    pub fn advance(&mut self) {
        self.cursor += 1;
    }

    /// decreases the cursor by one
    pub fn debase(&mut self) {
        if self.cursor == 0{
            return;
        }

        self.cursor -= 1;
    }

    pub fn core_dump(&self) -> &Box<[i64]>{
        self.cell.cells()
    }

    /// validation function for the input of brainf*ck program.
    fn validate(&self) -> Result<(), Error>{
        let mut stack: Vec<u8> = vec![];
        for c in self.input.bytes(){
            match c {
                b'[' => {
                    stack.push(c);
                },
                b']' => {
                    match stack.pop(){
                        None => {
                            return Err(Error("".to_string()));
                        },
                        _ => {}
                    }
                },
                _ => {
                    continue;
                }
            }
        }
        if ! stack.is_empty(){
            return Err(Error("".to_string()));
        }
        Ok(())
    }

    /// In the input this function is used
    /// to find the "twin" or match of the current
    /// square bracket and once found it sets the cursor
    /// to that position.
    pub fn jump_to(&mut self){
        // [---[>>>>]-]
        let current = self.cursor();
        let mut bytes = self.input.bytes();
        let mut seen = 0usize;

        let mut count = 0;
        while let Some(n) = bytes.next(){
            match n {
                b'[' => {
                    seen += 1;
                },

                b']' => {
                    if seen == 0 {
                        break;
                    } else{
                        seen -= 1;
                    }
                },
                _ => {
                }
            }
            count += 1;
        }
        self.cursor = current + count;
    }

    /// Just like the **jump_to** function
    /// this one does the opposite. Which
    /// is to find the *twin* match by travelling backward
    pub fn jump_back(&mut self) {
        let current = self.cursor;
        let chars = &self.input.
            chars().
            collect::<Vec<char>>()[0..current];

        let mut seen = 0;
        let mut len = chars.len() - 1;
        let mut count = 0;
        loop {
            let ch = chars[len];
            match ch {
                '[' => {
                    if seen != 0{
                        seen -= 1;
                    } else{
                        break;
                    }
                },
                ']' => {
                    seen += 1;
                },
                _ => {
                },
            }
            len -= 1;
            count += 1;
        }
        self.cursor = current - count;
    }

    /// Main execution begins here.
    /// Once the program input is loaded it
    /// starts execting here.
    pub fn execute(&mut self) -> AppResult {
        self.validate()?;
        while self.cursor() <= self.input.len() - 1 {
            match self.input.chars().nth(self.cursor()){
                Some(c) => {
                    match c {
                        '>' => {
                            match self.cell.next() {
                                Ok(()) => {},
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                            self.advance();
                        },
                        '<' => {
                            match self.cell.previous(){
                                Ok(()) => {},
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                            self.advance();
                        },
                        '+' => {
                            match self.cell.inc() {
                                Ok(())=>{},
                                Err(e) => return Err(e)
                            }
                            self.advance();
                        },
                        '-' => {
                            match self.cell.dec() {
                                Ok(())=>{},
                                Err(e) => return Err(e)
                            }
                            self.advance();

                        },
                        '.' => {
                            let c: char = (self.cell.data() as u8).into();
                            println!("{}", c);
                            self.advance();
                        },
                        ',' => {
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            if input.is_empty() {
                                self.cell.set(0);
                            } else{
                                let i = input.bytes().collect::<Vec<u8>>()[0];
                                self.cell.set(i as i64);
                            }

                            self.advance();
                        },

                        '[' => {
                            if self.cell.data() == 0 {
                                self.jump_to();
                            } else{
                                self.advance();
                            }
                        },

                        ']' => {
                            if self.cell.data() == 0{
                                self.advance();
                            } else {
                                // jump back to twin
                                self.jump_back();
                            }
                        },
                        _ => {
                            self.advance();
                        },
                    };
                },
                None => {
                    break;
                },
            }
        }
        Ok(())
    }
}

mod tests{
    #[test]
    fn test_validation() {
        let data = String::from("[>><<[]++--]");
        let program = crate::bf::prog::Program::new(data);
        assert_eq!(program.validate().ok(), Some(()));

        let data = String::from("[[]][][][][[]][][][]");
        let program = crate::bf::prog::Program::new(data);
        assert_eq!(program.validate().ok(), Some(()));

        let data = String::from("[>><<[]++--]]");
        let program = crate::bf::prog::Program::new(data);
        assert_eq!(program.validate().ok(), None);

        let data = String::from("][");
        let program = crate::bf::prog::Program::new(data);
        assert_eq!(program.validate().ok(), None);
    }

    #[test]
    fn test_memory_i(){
        let data = String::from("+++++");
        let mut program = crate::bf::Program::new(data);
        program.execute().ok();
        assert_eq!(program.cursor(), 5);
        assert_eq!(program.cell.ptr(), 0);
        assert_eq!(program.cell.cells()[0], 5);
    }
    
    #[test]
    fn test_memory_d(){
        let data = String::from("-----");
        let mut program = crate::bf::Program::new(data);
        program.execute().ok();
        assert_eq!(program.cursor(), 5);
        assert_eq!(program.cell.ptr(), 0);
        assert_eq!(program.cell.cells()[0], -5);
    }

    #[test]
    fn test_memory_ptr_n(){
        let data = String::from(">>>>>");
        let mut program = crate::bf::prog::Program::new(data);
        program.execute().ok();
        assert_eq!(program.cursor, 5);
        assert_eq!(program.cell.cells()[0], 0);
        assert_eq!(program.cell.ptr(), 5);
    }

    #[test]
    fn test_memory_ptr_p(){
        let data = String::from(">>>>><<<<<");
        let mut program = crate::bf::prog::Program::new(data);
        program.execute().ok();
        assert_eq!(program.cursor, 10);
        assert_eq!(program.cell.cells()[0], 0);
        assert_eq!(program.cell.ptr(), 0);
    }

    #[test]
    fn test_loop(){
        let data = String::from("+++++[>+++++<-]>");
        let len = data.len();
        let mut program = crate::bf::prog::Program::new(data);
        program.execute().ok();
        assert_eq!(program.cursor, len);
        assert_eq!(program.cell.cells()[0], 0);
        assert_eq!(program.cell.cells()[1], 25);
    }
}
