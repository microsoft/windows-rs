use test_winrt::{
    Windows::System::DispatcherQueueController,
    Windows::Win32::System::SystemServices::{
        CreateDispatcherQueueController, DispatcherQueueOptions,
        DISPATCHERQUEUE_THREAD_APARTMENTTYPE, DISPATCHERQUEUE_THREAD_TYPE,
    },
};

fn create_dispatcher() -> DispatcherQueueController {
    // We need a DispatcherQueue on our thread to properly create a Compositor. Note that since
    // we aren't pumping messages, the Compositor won't commit. This is fine for the test for now.

    let options = DispatcherQueueOptions {
        dwSize: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        threadType: DISPATCHERQUEUE_THREAD_TYPE::DQTYPE_THREAD_CURRENT,
        apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE::DQTAT_COM_NONE,
    };

    let mut interop = None;

    unsafe {
        CreateDispatcherQueueController(options, &mut interop)
            .and_some(interop)
            .unwrap()
    }
}

#[test]
fn class_hierarchy_conversion() -> windows::Result<()> {
    use test_winrt::Windows::UI::Composition::{Compositor, SpriteVisual, Visual};

    let _dispatcher = create_dispatcher();
    let compositor = Compositor::new()?;

    // Convert from SpriteVisual class to base Visual class by value (dropping the sprite).
    let sprite: SpriteVisual = compositor.CreateSpriteVisual()?;
    sprite.SetComment("test")?;
    let visual: Visual = sprite.into();
    assert!(visual.Comment()? == "test");

    // Convert from SpriteVisual class to base Visual class by reference (retaining the sprite).
    let sprite: &SpriteVisual = &compositor.CreateSpriteVisual()?;
    sprite.SetComment("test")?;
    let visual: Visual = sprite.into();
    assert!(visual.Comment()? == "test");
    assert!(visual.Comment()? == sprite.Comment()?);

    // Convert from SpriteVisual class to base Visual class *parameter* by value (dropping the sprite).
    let container = compositor.CreateContainerVisual()?;
    let children = container.Children()?;
    let sprite: SpriteVisual = compositor.CreateSpriteVisual()?;
    sprite.SetComment("test")?;
    children.InsertAtBottom(sprite)?;
    assert!(children.First()?.Current()?.Comment()? == "test");

    // Convert from SpriteVisual class to base Visual class *parameter* by reference (retaining the sprite).
    let container = compositor.CreateContainerVisual()?;
    let children = container.Children()?;
    let sprite: &SpriteVisual = &compositor.CreateSpriteVisual()?;
    sprite.SetComment("test")?;
    children.InsertAtBottom(sprite)?;
    assert!(children.First()?.Current()?.Comment()? == "test");
    assert!(children.First()?.Current()?.Comment()? == sprite.Comment()?);

    Ok(())
}

#[test]
fn composition() -> windows::Result<()> {
    use test_winrt::Windows::Foundation::Numerics::Vector3;
    use test_winrt::Windows::UI::Composition::{CompositionColorBrush, Compositor};
    use test_winrt::Windows::UI::{Color, Colors};
    use windows::Interface;

    let _dispatcher = create_dispatcher();
    let compositor = Compositor::new()?;
    let visual = compositor.CreateSpriteVisual()?;
    let red = Colors::Red()?;

    assert!(
        red == Color {
            A: 255,
            R: 255,
            G: 0,
            B: 0
        }
    );

    // Visual.set_brush expects a CompositionBrush but CreateColorBrushWithColor returns a
    // CompositionColorBrush that logically derives from CompositionBrush.
    let brush = compositor.CreateColorBrushWithColor(&red)?;
    visual.SetBrush(brush)?;

    // Visual.brush returns a CompositionBrush but we know that it's actually a CompositionColorBrush
    // and need to convert it excplicitly since Rust/WinRT doesn't know that.
    let brush: CompositionColorBrush = visual.Brush()?.cast()?;
    assert!(brush.Color()? == red);

    visual.SetOffset(Vector3 {
        X: 1.0,
        Y: 2.0,
        Z: 3.0,
    })?;

    assert!(
        visual.Offset()?
            == Vector3 {
                X: 1.0,
                Y: 2.0,
                Z: 3.0
            }
    );

    let children = visual.Children()?;

    let child = compositor.CreateSpriteVisual()?;
    child.SetOffset(Vector3 {
        X: 1.0,
        Y: 0.0,
        Z: 0.0,
    })?;
    children.InsertAtBottom(child)?;

    let child = compositor.CreateSpriteVisual()?;
    child.SetOffset(Vector3 {
        X: 2.0,
        Y: 0.0,
        Z: 0.0,
    })?;
    children.InsertAtBottom(child)?;

    let child = compositor.CreateSpriteVisual()?;
    child.SetOffset(Vector3 {
        X: 3.0,
        Y: 0.0,
        Z: 0.0,
    })?;
    children.InsertAtBottom(child)?;

    assert!(children.Count()? == 3);

    // TODO: Collection iteration is still crude but at least the underlying collection interfaces are working.

    let iterator = children.First()?;
    assert!(iterator.HasCurrent()?);

    assert!(
        iterator.Current()?.Offset()?
            == Vector3 {
                X: 3.0,
                Y: 0.0,
                Z: 0.0,
            }
    );

    assert!(iterator.MoveNext()?);
    assert!(iterator.HasCurrent()?);

    assert!(
        iterator.Current()?.Offset()?
            == Vector3 {
                X: 2.0,
                Y: 0.0,
                Z: 0.0,
            }
    );

    assert!(iterator.MoveNext()?);
    assert!(iterator.HasCurrent()?);

    assert!(
        iterator.Current()?.Offset()?
            == Vector3 {
                X: 1.0,
                Y: 0.0,
                Z: 0.0,
            }
    );

    assert!(iterator.MoveNext()? == false);
    assert!(iterator.HasCurrent()? == false);

    Ok(())
}
