use winrt_macros::*;

#[echo_target]
pub trait Stringable {
    fn to_string();
}

#[derive(winrt_macros::Stringable)]
pub struct Apples;

#[derive(Stringable)]
pub struct Oranges;

#[replace_your_innards]
pub fn change()
{
    println!("change!");
}
