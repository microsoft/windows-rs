use winrt_macros::Stringable;

pub trait Stringable {
    fn to_string();
}

#[derive(winrt_macros::Stringable)]
pub struct Apples;

#[derive(Stringable)]
pub struct Oranges;
