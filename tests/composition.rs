winrt::import!(
    dependencies
        "os"
    modules
        "windows.ui.composition"
);

use windows::foundation::collections::*;
use windows::ui::composition::*;
use winrt::*;

// impl<T: RuntimeType> Iterator for IIterator<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         let result = self.current().ok();

//         if result.is_some() {
//             self.move_next().ok()?;
//         }

//         result
//     }
// }

impl IntoIterator for &VisualCollection {
    type Item = Visual;
    type IntoIter = IIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.first().unwrap()
    }
}

#[test]
fn uri() -> winrt::Result<()> {
    if false {
        use windows::foundation::numerics::*;
        use windows::ui::composition::*;
        use windows::ui::*;
        use winrt::*;

        let compositor = Compositor::new()?;
        let visual = compositor.create_sprite_visual()?;

        // Hidden QI to IVisual happens before calling set_offset
        visual.set_offset(Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0
        })?;



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

        for v in &children {}

        let mut iterator = children.into_iter();

        assert!(
            iterator.next().unwrap().offset()?
                == Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                }
        );

        assert!(
            iterator.next().unwrap().offset()?
                == Vector3 {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                }
        );

        assert!(
            iterator.next().unwrap().offset()?
                == Vector3 {
                    x: 3.0,
                    y: 0.0,
                    z: 0.0,
                }
        );

        assert!(iterator.next().is_none() == true);
    }

    Ok(())
}
