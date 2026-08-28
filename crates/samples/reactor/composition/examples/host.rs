#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_composition::{Color, Compositor, ContainerVisual, SpriteVisual};
use windows_core::Result;
use windows_reactor::*;

struct Scene {
    root: ContainerVisual,
    background: SpriteVisual,
    square: SpriteVisual,
}

impl Scene {
    fn build(
        compositor: windows_core::IUnknown,
        host: &ElementRef<Grid>,
        width: f32,
        height: f32,
    ) -> Result<Self> {
        let compositor = Compositor::from_host(compositor)?;

        let root = compositor.create_container_visual();
        let background = compositor.create_sprite_visual();
        background.set_brush(&compositor.create_color_brush(Color::rgb(24, 24, 32)));
        root.children().insert_at_bottom(&background);

        let square = compositor.create_sprite_visual();
        square.set_size(160.0, 160.0);
        square.set_brush(&compositor.create_color_brush(Color::rgb(0, 120, 215)));
        root.children().insert_at_top(&square);

        let _ = host.request_set_child_visual(Some(root.as_raw().into()), |result| {
            if let Err(error) = result {
                eprintln!("failed to attach composition visual: {error:?}");
            }
        });
        let scene = Self {
            root,
            background,
            square,
        };
        scene.layout(width, height)?;
        Ok(scene)
    }

    fn layout(&self, width: f32, height: f32) -> Result<()> {
        self.root.set_size(width, height);
        self.background.set_size(width, height);
        let size = self.square.size();
        self.square
            .set_offset((width - size.x) / 2.0, (height - size.y) / 2.0, 0.0);
        Ok(())
    }
}

struct Sample {
    host: ElementRef<Grid>,
    scene: Rc<RefCell<Option<Scene>>>,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            host: ElementRef::new(),
            scene: Rc::new(RefCell::new(None)),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Composition Host");
        let host = self.host.clone();
        let scene = Rc::clone(&self.scene);
        context.use_effect("composition-host", (), move || {
            let event_host = host.clone();
            let observation = host.observe_composition_host(move |event| match event {
                CompositionHostEvent::Ready {
                    compositor,
                    width,
                    height,
                    ..
                } => match Scene::build(compositor, &event_host, width as f32, height as f32) {
                    Ok(built) => *scene.borrow_mut() = Some(built),
                    Err(error) => eprintln!("composition init failed: {error}"),
                },
                CompositionHostEvent::Metrics { width, height, .. } => {
                    if let Some(scene) = scene.borrow().as_ref() {
                        scene.layout(width as f32, height as f32).unwrap();
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
