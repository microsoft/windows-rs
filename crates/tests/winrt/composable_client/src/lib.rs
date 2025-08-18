#![cfg(target_env = "msvc")]
#![cfg(test)]

mod bindings;
use bindings::*;
use windows::core::*;

unsafe extern "system" {
    fn interop() -> HRESULT;
}

#[test]
fn test() -> Result<()> {
    unsafe { interop().ok()? };

    let compositor = Compositor::new()?;

    let container = compositor.CreateContainerVisual(123)?;
    assert_eq!(container.Children(), 123);
    assert_eq!(container.Compositor()?, compositor);

    let sprite = compositor.CreateSpriteVisual(456)?;
    assert_eq!(sprite.Brush(), 456);
    assert_eq!(sprite.Children(), 456 * 2);
    assert_eq!(sprite.Compositor()?, compositor);

    Ok(())
}
