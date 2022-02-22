#[derive(Copy, Clone)]
pub enum ArrayInfo {
    Fixed(usize),
    RelativeSize(usize),
    RelativePtr(usize),
}
