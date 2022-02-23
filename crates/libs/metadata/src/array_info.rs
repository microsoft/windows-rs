#[derive(Copy, Clone)]
pub enum ArrayInfo {
    Fixed(usize),
    RelativeLen(usize),
    RelativeSize(usize),
    RelativePtr(usize),
}
