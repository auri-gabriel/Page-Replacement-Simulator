use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod page;
use page::Page;

pub struct Memory {
    size: usize,
    frame_size: usize,
    memory: VecDeque<Page>,
    pub page_faults: usize,
}

const PAGE_SIZE: usize = 4096;

impl Memory {
    pub fn new(size: usize, frame_size: usize) -> Self {
        Memory {
            size,
            frame_size,
            memory: VecDeque::new(),
            page_faults: 0,
        }
    }

    pub fn simular(&mut self, filename: &str, algorithm: &str) {
        todo!()
    }
}
