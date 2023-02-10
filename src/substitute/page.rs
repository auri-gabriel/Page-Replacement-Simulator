use std::time::Instant;

#[derive(Clone, Copy)]
pub struct Page {
    pub page_number: i32,
    pub frame_number: i32,
    pub last_access: Instant,
    pub referenced: bool,
    pub dirty: bool,
}

impl Page {
    pub fn new(page_number: i32, frame_number: i32) -> Self {
        Page {
            page_number,
            frame_number,
            last_access: Instant::now(),
            referenced: true,
            dirty: false,
        }
    }
}
