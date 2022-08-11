mod utils;

use wasm_bindgen::prelude::*;
use std::convert::From;
use std::error::Error;
use std::fmt;
type Res<T> = Result<T, &'static str>;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

use Cell::*;

impl Cell {
    fn symbol(&self) -> char {
        match self {
            Dead => '⬜',
            Alive => '⬛'
        }
    }
}

impl From<bool> for Cell {
    fn from(b: bool) -> Self {
        if b {Alive} else {Dead}
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl Universe {

    pub fn new() -> Self {
        let (height, width) = (64, 64);
        
        Universe { 
            width: 64, height: 64, 
            cells: (0..width * height).map(|i| {
                Cell::from(i % 2 == 0 || i % 5 == 0)
            }).collect()
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, col: u32) -> Res<usize>  {
        if row >= self.height || col >= self.width {
            Err("Out of bounds")
        } else {
            Ok((row * self.width + col) as usize)

        }
    }

    fn get_rc(&self, idx: usize) -> Res<(u32, u32)> {
        if idx >= self.cells.len() {
            Err("Out of bounds")
        } else {
            Ok((idx as u32 / self.width, idx as u32 % self.width))
        }
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u32 {
        let mut cnt = 0;
        for row_iter in row as i32-1..=row as i32+1 {
            for col_iter in col - 1..=col+1 {
                let (r, c) = (row_iter as u32 % self.height, col_iter as u32 % self.width);

                if r  == row  && c == col {
                    continue;
                }
                cnt += self.cells[self.get_index(r, c).unwrap()] as u32;
            }
        }
        cnt
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for idx in  0..self.cells.len() {
            let (r, c) = self.get_rc(idx).unwrap();
            let neighbor_cnt = self.live_neighbor_count(r, c);

            next[idx] = match self.cells[idx] {
                Dead => Cell::from(self.live_neighbor_count(r, c) == 3) ,
                Alive => Cell::from(neighbor_cnt == 2 || neighbor_cnt == 3)
            };
        }
        self.cells = next;
    }

}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for idx in 0..self.cells.len() {
            if idx % self.width as usize == 0 && idx > 0 {
                write!(f,"\n")?;
            }
            write!(f, "{}", self.cells[idx].symbol())?;
        }

        Ok(())
    }
}
