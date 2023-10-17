#[derive(Default)]
pub struct Column {
    pub offset: usize,
    pub width: usize,
}

impl Column {
    pub fn new(offset: usize, width: usize) -> Self {
        Self { offset, width }
    }
}
