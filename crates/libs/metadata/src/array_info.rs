#[derive(Copy, Clone)]
pub enum ArrayInfo {
    Relative(usize),
    Fixed(usize),
}
