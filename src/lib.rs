//! brainf*ck implementation for training purpose
//! Basic implementation of the brainf*ck interpreter
//! module bf
pub mod bf;

pub mod bf_help{
    pub fn show_full_help() -> String {
        r#"
        THE BRAIN F*CK INTERPRETER
        A simple easy to use interpreter for simulating
        the brainf*ck esotric programming language first
        designed by **.

        To get started it can be easily used the REPL
        in which a brainf*ck program can be easily typed in
        and evaluated on the fly.

        Otherwise this program can be used by providing different
        arguments in order to run a program from a file and
        check the memory layout.
        "#.into()
    }
}

pub mod bf_runner {
    use std::fmt::Write as _;
    use std::io::Write as _;

    use std::io::Write;

    use crate::bf::{AppResult, prog::Program};
    pub enum Base {
        Hexadecimal(u8),
        Decimal(u8),
        Binary(u8),
    }

    impl From<u8> for Base{
        fn from(value: u8) -> Self {
            match value {
                2 => Base::Binary(value),
                10 => Base::Decimal(value),
                0x10 => Base::Hexadecimal(value),
                _ => { Base::Decimal(10)}
            }
        }
    }

    pub struct Runner<'a> {
        base: Option<crate::bf_runner::Base>,
        dump: Option<String>,
        file: &'a std::path::Path,
    }

    impl<'a> Runner<'a> {
        pub fn new(
            base: Option<crate::bf_runner::Base>,
            dump: Option<String>,
            path: &'a String,
        ) -> Runner {
            Runner {
                base,
                dump,
                file: std::path::Path::new(path),
            }
        }

        pub fn execute(&mut self) -> AppResult {
            let buffer = String::from_utf8(*self.read_input()).unwrap();
            let mut program = Program::new(buffer);
            program.execute()?;
            if let Some(dmp) = &self.dump{
                let core_dmp = dmp;
                let cells = program.core_dump();
                let base = match &self.base {
                    Some(b) => { b },
                    None => { &Base::Decimal(10) }
                };
                self.write_into(cells, core_dmp, base)?;
            }
            Ok(())
        }

        fn write_into(&self, cells: &Box<[i64]>, core: &String, base: &Base) -> AppResult{
            let mut file = std::fs::File::create(core).unwrap();
            let base = match *base {
                Base::Binary(b) => b,
                Base::Decimal(b) => b,
                Base::Hexadecimal(b) =>b
            };

            for i in 0..cells.len() {
                let cell = cells[i];
                write!(file, "{}", self.format_cell(cell, base)).unwrap();
            }

            Ok(())
        }

        fn format_cell(&self, cell: i64, base: u8) -> String {
            // right here
            // must be written in a formatted way
            let mut res = String::new();
            write!(&mut res, "{}", cell>>(56)).unwrap();
            res
        }


        fn read_input(&self) -> Box<Vec<u8>>{
            Box::new(std::fs::read(self.file).unwrap())
        }
    }
}

mod tests{
}
