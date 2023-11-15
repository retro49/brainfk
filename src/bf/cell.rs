use crate::bf::Error;

type MemoryCell = Box<[i64]>;
/// Memory cell
/// This is where the cell is created and managed
/// by bf.
/// visualizaion...
///  |---|---|---|---|---|---|---|---|
///  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
///  |---|---|---|---|---|---|---|---|
///    ^                            
///    |
///   ptr
pub struct Cell {
    cells: MemoryCell,
    ptr: usize,
}

pub type CellResult = Result<(), crate::bf::Error>;

impl Cell {
    /// creates a new memory cell
    pub fn new() -> Self {
        Cell {
            cells: Box::new([0;crate::bf::BF_DEFAULT_CELL]),
            ptr: 0,
        }
    }

    /// returns the size of memory cell
    /// which is 30000 by default.
    ///  |---|---|---|---|---|---|---|---|
    ///  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    ///  |---|---|---|---|---|---|---|---|
    ///    ^                            * len = 8
    ///    |
    ///   ptr
    pub fn len(&self) -> usize{
        self.cells.len()
    }


    /// increases the current cell by one.
    ///
    ///  |---|---|---|---|---|---|---|---|
    ///  | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    ///  |---|---|---|---|---|---|---|---|
    ///    ^                            
    ///    |
    ///   ptr
    pub fn inc(&mut self) -> CellResult{
        if self.cells[self.ptr] > i64::MAX{
            return Err(crate::bf::Error("".to_string()));
        }
        self.cells[self.ptr] += 1;
        Ok(())
    }

    /// decreases the current cell by one.
    ///  |---|---|---|---|---|---|---|---|
    ///  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    ///  |---|---|---|---|---|---|---|---|
    ///    ^                           
    ///    |
    ///   ptr
    pub fn dec(&mut self) -> CellResult{
        if self.cells[self.ptr] < i64::MIN{
            return Err(crate::bf::Error("".to_string()));
        }
        self.cells[self.ptr] -= 1;
        Ok(())
    }

    /// returns the pointer to the current cell
    ///  |---|---|---|---|---|---|---|---|
    ///  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    ///  |---|---|---|---|---|---|---|---|
    ///    ^                            
    ///    |
    ///   *ptr

    pub fn ptr(&self) -> usize{
        self.ptr
    }

    ///  |---|---|---|---|---|---|---|---|
    ///  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    ///  |---|---|---|---|---|---|---|---|
    ///        ^                            
    ///        |
    ///      *ptr
    pub fn next(&mut self) -> CellResult {
        if self.ptr >= self.cells.len() - 1 {
            return Err(Error("no more cell to advance".to_string()));
        }
        self.ptr += 1usize;
        Ok(())
    }

    ///  rewinds to the previous cell.
    ///  While the pointer is on cell 0, trying to 
    ///  rewind will cause an error, please beware of that.
    ///  |---|---|---|---|---|---|---|---|
    ///  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    ///  |---|---|---|---|---|---|---|---|
    ///    ^                            
    ///    |
    ///   *ptr
    pub fn previous(&mut self) -> CellResult{
        if self.ptr == 0{
            return Err(Error("no more previous memory".to_string()));
        }
        self.ptr -= 1usize;
        Ok(())
    }

    pub fn data(&self) -> i64{
        self.cells[self.ptr]
    }

    pub fn set(&mut self, b: i64) {
        self.cells[self.ptr] = b;
    }

    pub fn cells<'a>(&'a self) -> &'a MemoryCell {
        &self.cells
    }
}


pub mod tests{

    #[test]
    fn mem_test(){
        let mut cell: crate::bf::cell::Cell = 
            crate::bf::cell::Cell::new();
        assert!(cell.ptr() == 0);
        assert_eq!(cell.len(), 30_000);
        for _ in 0..10{
            cell.inc().unwrap();
        }
        assert_eq!(cell.data(), 10);

        for _ in 0..10{
            cell.dec().unwrap();
        }
        assert_eq!(cell.data(), 0);

        cell.next().unwrap();
        assert_eq!(cell.ptr(), 1);

        cell.previous().unwrap();
        assert_eq!(cell.ptr(), 0);
    }
}
