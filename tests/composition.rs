winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui.composition"
);

#[link(name = "coremessaging")]
extern "stdcall" {
    fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        dispatcherQueueController: *mut winrt::ComInterfacePtr<winrt::IUnknown>,
    ) -> winrt::ErrorCode;
}

#[repr(C)]
struct DispatcherQueueOptions {
    size: u32,
    thread_type: i32,
    apartment_type: i32,
}

fn create_dispatcher() -> winrt::IUnknown {
    // We need a DispatcherQueue on our thread to properly create a Compositor. Note that since
    // we aren't pumping messages, the Compositor won't commit. This is fine for the test for now.

    let options = DispatcherQueueOptions {
        size: std::mem::size_of::<DispatcherQueueOptions>() as u32,
        thread_type: 2,    // DQTYPE_THREAD_CURRENT
        apartment_type: 0, // DQTAT_COM_NONE
    };

    let mut interop_ptr = winrt::IUnknown::default();
    unsafe {
        CreateDispatcherQueueController(options, interop_ptr.set_abi())
            .ok()
            .unwrap();
    }
    interop_ptr
}

#[test]
fn class_hierarchy_conversion() -> winrt::Result<()> {
    use windows::ui::composition::{Compositor, SpriteVisual, Visual};

    let _dispatcher = create_dispatcher();
    let compositor = Compositor::new()?;

    // Convert from SpriteVisual class to base Visual class by value (dropping the sprite).
    let sprite: SpriteVisual = compositor.create_sprite_visual()?;
    sprite.set_comment("test")?;
    let visual: Visual = sprite.into();
    assert!(visual.comment()? == "test");

    // Convert from SpriteVisual class to base Visual class by reference (retaining the sprite).
    let sprite: &SpriteVisual = &compositor.create_sprite_visual()?;
    sprite.set_comment("test")?;
    let visual: Visual = sprite.into();
    assert!(visual.comment()? == "test");
    assert!(visual.comment()? == sprite.comment()?);

    // Convert from SpriteVisual class to base Visual class *parameter* by value (dropping the sprite).
    let container = compositor.create_container_visual()?;
    let children = container.children()?;
    let sprite: SpriteVisual = compositor.create_sprite_visual()?;
    sprite.set_comment("test")?;
    children.insert_at_bottom(sprite)?;
    assert!(children.first()?.current()?.comment()? == "test");

    // Convert from SpriteVisual class to base Visual class *parameter* by reference (retaining the sprite).
    let container = compositor.create_container_visual()?;
    let children = container.children()?;
    let sprite: &SpriteVisual = &compositor.create_sprite_visual()?;
    sprite.set_comment("test")?;
    children.insert_at_bottom(sprite)?;
    assert!(children.first()?.current()?.comment()? == "test");
    assert!(children.first()?.current()?.comment()? == sprite.comment()?);

    Ok(())
}

#[test]
fn composition() -> winrt::Result<()> {
    use windows::foundation::numerics::Vector3;
    use windows::ui::composition::{CompositionColorBrush, Compositor};
    use windows::ui::{Color, Colors};
    use winrt::TryInto;

    let _dispatcher = create_dispatcher();
    let compositor = Compositor::new()?;
    let visual = compositor.create_sprite_visual()?;
    let red = Colors::red()?;

    assert!(
        red == Color {
            a: 255,
            r: 255,
            g: 0,
            b: 0
        }
    );

    // Visual.set_brush expects a CompositionBrush but create_color_brush_with_color returns a
    // CompositionColorBrush that logically derives from CompositionBrush.
    let brush = compositor.create_color_brush_with_color(&red)?;
    visual.set_brush(brush)?;

    // Visual.brush returns a CompositionBrush but we know that it's actually a CompositionColorBrush
    // and need to convert it excplicitly since Rust/WinRT doesn't know that.
    let brush: CompositionColorBrush = visual.brush()?.try_into().unwrap();
    assert!(brush.color()? == red);

    visual.set_offset(Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    })?;

    assert!(
        visual.offset()?
            == Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
    );

    let children = visual.children()?;

    let child = compositor.create_sprite_visual()?;
    child.set_offset(Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    })?;
    children.insert_at_bottom(child)?;

    let child = compositor.create_sprite_visual()?;
    child.set_offset(Vector3 {
        x: 2.0,
        y: 0.0,
        z: 0.0,
    })?;
    children.insert_at_bottom(child)?;

    let child = compositor.create_sprite_visual()?;
    child.set_offset(Vector3 {
        x: 3.0,
        y: 0.0,
        z: 0.0,
    })?;
    children.insert_at_bottom(child)?;

    assert!(children.count()? == 3);

    // TODO: Collection iteration is still crude but at least the underlying collection interfaces are working.

    let iterator = children.first()?;
    assert!(iterator.has_current()?);

    assert!(
        iterator.current()?.offset()?
            == Vector3 {
                x: 3.0,
                y: 0.0,
                z: 0.0,
            }
    );

    assert!(iterator.move_next()?);
    assert!(iterator.has_current()?);

    assert!(
        iterator.current()?.offset()?
            == Vector3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            }
    );

    assert!(iterator.move_next()?);
    assert!(iterator.has_current()?);

    assert!(
        iterator.current()?.offset()?
            == Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
    );

    assert!(iterator.move_next()? == false);
    assert!(iterator.has_current()? == false);

    Ok(())
}
