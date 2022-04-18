use windows::{core::*,Foundation::*};

#[test]
fn test() -> Result<()> {
    let mut event = Event::<EventHandler<i32>>::new();
    event.add(&EventHandler::<i32>::new(|_, args| {Ok(())}))?;
    Ok(())
}

