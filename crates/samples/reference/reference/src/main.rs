use windows_core::Result;
use windows_reference::*;
use windows_strings::*;

fn main() -> Result<()> {
    let count = IReference::<i32>::from(42);
    let message = IReference::<HSTRING>::from("hello");

    println!("count: {}", count.Value()?);
    println!("message: {}", message.Value()?.to_string_lossy());

    Ok(())
}
