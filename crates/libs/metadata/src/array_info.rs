#[derive(Copy, Clone)]
pub enum ArrayInfo {
    Relative(i16),
    Fixed(i32),
}
