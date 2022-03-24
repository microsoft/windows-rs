use std::slice::*;
use std::mem::*;

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

pub trait Write {
    fn write<T: Sized>(&mut self, value: &T);
}

impl Write for Vec<u8> {
    fn write<T: Sized>(&mut self, value: &T) {
        unsafe {
            self.extend_from_slice(from_raw_parts(value as *const _ as _, size_of::<T>()));
        }
    }
}
