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

        let brush = compositor.create_color_brush_with_color(red)?;
        visual.set_brush(brush)?;

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
    }

    Ok(())
}
