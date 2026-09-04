#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use windows_composition::{CompositionColor, Compositor, SpriteVisual};
use windows_core::Result;
use windows_numerics::Vector3;
use windows_reactor::*;

const SIZE: f32 = 160.0;

fn build(compositor: windows_core::IUnknown, host: &ElementRef<Grid>) -> Result<SpriteVisual> {
    let compositor = Compositor::from_host(compositor)?;
    let visual = compositor.create_sprite_visual();
    visual.set_size(SIZE, SIZE);
    visual.set_center_point(Vector3 {
        x: SIZE / 2.0,
        y: SIZE / 2.0,
        z: 0.0,
    });
    visual.set_brush(&compositor.create_color_brush(CompositionColor::rgb(0, 153, 188)));

    let pulse = compositor.create_vector3_key_frame_animation();
    pulse.insert_key_frame(
        0.0,
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    );
    pulse.insert_key_frame(
        0.5,
        Vector3 {
            x: 1.5,
            y: 1.5,
            z: 1.0,
        },
    );
    pulse.insert_key_frame(
        1.0,
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    );
    pulse.set_duration(Duration::from_millis(1500));
    pulse.set_iterate_forever();
    visual.start_animation("Scale", &pulse);
    let _ = host.request_set_child_visual(Some(visual.as_raw().into()), |_| {});
    Ok(visual)
}

struct Sample {
    host: ElementRef<Grid>,
    visual: Rc<RefCell<Option<SpriteVisual>>>,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            host: ElementRef::new(),
            visual: Rc::new(RefCell::new(None)),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Composition Animation");
        let host = self.host.clone();
        let visual = Rc::clone(&self.visual);
        context.use_effect("composition-host", (), move || {
            let event_host = host.clone();
            let observation = host.observe_composition_host(move |event| match event {
                CompositionHostEvent::Ready {
                    compositor,
                    width,
                    height,
                    ..
                } => match build(compositor, &event_host) {
                    Ok(built) => {
                        built.set_offset(
                            (width as f32 - SIZE) / 2.0,
                            (height as f32 - SIZE) / 2.0,
                            0.0,
                        );
                        *visual.borrow_mut() = Some(built);
                    }
                    Err(error) => eprintln!("composition init failed: {error}"),
                },
                CompositionHostEvent::Metrics { width, height, .. } => {
                    if let Some(visual) = visual.borrow().as_ref() {
                        visual.set_offset(
                            (width as f32 - SIZE) / 2.0,
                            (height as f32 - SIZE) / 2.0,
                            0.0,
                        );
                    }
                }
            });
            Some(Box::new(move || drop(observation)))
        });
        Grid::new().element_ref(&self.host).into()
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
