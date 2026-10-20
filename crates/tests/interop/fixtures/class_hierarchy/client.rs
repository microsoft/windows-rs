use super::*;
use ClassHierarchy::*;

pub fn test() -> Result<()> {
    let compositor = Compositor::new()?;

    let container = compositor.CreateContainerVisual(123)?;
    assert_eq!(container.Children(), 123);
    assert_eq!(container.Compositor()?, compositor);

    let sprite = compositor.CreateSpriteVisual(456)?;
    assert_eq!(sprite.Brush(), 456);
    assert_eq!(sprite.Children(), 912);
    assert_eq!(sprite.Compositor()?, compositor);

    let visual = compositor.Echo(&sprite)?;
    assert_eq!(visual.cast::<SpriteVisual>()?, sprite);
    assert_eq!(visual.cast::<SpriteVisual>()?.Brush(), 456);
    assert!(compositor.Echo(&container)?.cast::<SpriteVisual>().is_err());

    Ok(())
}
