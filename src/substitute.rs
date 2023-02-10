use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::time::Instant;

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

    pub fn simulate(&mut self, filename: &str, algorithm: &str) {
        let file = File::open(filename).expect("File not found");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            let address = usize::from_str_radix(parts[0], 16).unwrap();
            let page_number = address / PAGE_SIZE;
            let frame_number = address / self.frame_size;
            let mut found = false;

            for page in &mut self.memory {
                if page.frame_number == frame_number.try_into().unwrap()
                    && page.page_number == page_number.try_into().unwrap()
                {
                    found = true;
                    if algorithm == "LRU" {
                        page.last_access = Instant::now();
                    }
                    if parts[1] == "W" {
                        page.dirty = true;
                    }
                    break;
                }
            }

            if !found {
                if self.memory.len() == self.size {
                    let page_to_remove = match algorithm {
                        "LRU" => self.remove_page_by_lru(),
                        "FIFO" => self.remove_page_by_fifo(),
                        "VMS" => self.remove_page_by_vms(),
                        _ => panic!("Invalid algorithm"),
                    };

                    self.memory.retain(|p| {
                        p.page_number != page_to_remove.page_number
                            || p.frame_number != page_to_remove.frame_number
                    });
                    self.page_faults += 1;
                }

                self.memory
                    .push_back(Page::new(frame_number as i32, page_number as i32));
                if parts[1] == "W" {
                    let last = self.memory.back_mut().unwrap();
                    last.dirty = true;
                }
            }
        }
    }

    fn remove_page_by_lru(&mut self) -> Page {
        self.memory
            .iter()
            .min_by_key(|p| p.last_access)
            .unwrap()
            .clone()
    }

    fn remove_page_by_fifo(&mut self) -> Page {
        self.memory.pop_front().unwrap()
    }

    fn remove_page_by_vms(&mut self) -> Page {
        for p in &mut self.memory {
            p.referenced = false;
        }
        self.memory
            .iter()
            .min_by_key(|p| p.last_access)
            .unwrap()
            .clone()
    }
}
