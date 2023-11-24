//! brainf*ck implementation for training purpose
//! Basic implementation of the brainf*ck interpreter
//! module bf
pub mod bf;

pub mod bf_runner {

    use std::io::Write;

    use crate::bf::{AppResult, prog::Program};
    pub enum Base {
        Binary(u8),
        Octal(u8),
        Decimal(u8),
        Hexadecimal(u8)
    }

    impl From<u8> for Base{
        fn from(value: u8) -> Self {
            match value {
                2 => Base::Binary(value),
                8 => Base::Octal(value),
                10 => Base::Decimal(value),
                0x10 => Base::Hexadecimal(value),
                _ => { Base::Decimal(10) }
            }
        }
    }

    pub struct Runner<'a> {
        base: Option<crate::bf_runner::Base>,
        core: Option<String>,
        file: &'a std::path::Path,
    }

    impl<'a> Runner<'a> {
        pub fn new(
            base: Option<crate::bf_runner::Base>,
            core: Option<String>,
            path: &'a String,
        ) -> Runner {
            Runner {
                base,
                core,
                file: std::path::Path::new(path),
            }
        }

        pub fn execute(&mut self) -> AppResult {
            let buffer = String::from_utf8(*self.read_input()).unwrap();
            let mut program = Program::new(buffer);
            program.execute()?;
            if let Some(dmp) = &self.core{
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
                Base::Octal(b) => b,
                Base::Decimal(b) => b,
                Base::Hexadecimal(b) =>b,
            };

            for i in 0..cells.len() {
                let cell = cells[i];
                match cell {
                    0 => {
                        writeln!(file, "{}", "..").unwrap();
                    },
                    _ => {
                        writeln!(file, "{}", self.format_cell(cell, base)).unwrap();
                    }
                };
            }
            Ok(())
        }

        fn format_cell(&self, cell: i64, base: u8) -> String {
            let mut res = String::new();
            let mut cell = cell;
            while cell != 0 {
                let mut frmt = match base {
                    2 => format!("{:b}", cell as u8),
                    8 => format!("{:o}", cell as u8),
                    10 => format!("{}", cell as u8),
                    0x10 => format!("{:x}", cell as u8),
                    _ => format!("{}", cell as u8),
                };

                if frmt.len() < 2 {
                    frmt = "0".to_string() + &frmt;
                }
                res.push_str(frmt.as_str());
                res.push(' ');
                cell >>= 8;
            }
            res
        }

        fn read_input(&self) -> Box<Vec<u8>> {
            Box::new(std::fs::read(self.file).unwrap())
        }
    }
}

mod tests{
    #[test]
    fn test_cell_format(){
    }
}
