#[derive(Copy, Clone)]
pub enum ArrayInfo {
    Fixed(usize),
    RelativeLen(usize),
    RelativePtr(Option<usize>),
}
