#[derive(Default)]
pub struct Table {
    pub offset: usize,
    pub len: usize,
    pub width: usize,
    pub columns: [Column; 6],
}

#[derive(Default)]
pub struct Column {
    pub offset: usize,
    pub width: usize,
}

impl Table {
    pub fn index_width(&self) -> usize {
        if self.len < (1 << 16) {
            2
        } else {
            4
        }
    }

    pub fn set_columns(&mut self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) {
        self.width = a + b + c + d + e + f;
        self.columns[0] = Column::new(0, a);
        if b != 0 {
            self.columns[1] = Column::new(a, b);
        }
        if c != 0 {
            self.columns[2] = Column::new(a + b, c);
        }
        if d != 0 {
            self.columns[3] = Column::new(a + b + c, d);
        }
        if e != 0 {
            self.columns[4] = Column::new(a + b + c + d, e);
        }
        if f != 0 {
            self.columns[5] = Column::new(a + b + c + d + e, f);
        }
    }

    pub fn set_data(&mut self, offset: &mut usize) {
        if self.len != 0 {
            let next = *offset + self.len * self.width;
            self.offset = *offset;
            *offset = next;
        }
    }
}

impl Column {
    fn new(offset: usize, width: usize) -> Self {
        Self { offset, width }
    }
}
