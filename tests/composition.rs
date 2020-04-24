winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui.composition"
);

#[test]
fn uri() -> winrt::Result<()> {
    if false {
        use windows::foundation::numerics::*;
        use windows::ui::composition::*;
        use windows::ui::*;
        use winrt::*;

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
                    x: 1.0,
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
                    x: 3.0,
                    y: 0.0,
                    z: 0.0,
                }
        );

        assert!(iterator.move_next()? == false);
        assert!(iterator.has_current()? == false);
    }

    Ok(())
}
