#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_composition::*;
use windows_core::*;
use windows_reactor::*;

struct Scene {
    root: ContainerVisual,
    background: SpriteVisual,
    square: SpriteVisual,
}

impl Scene {
    fn build(
        compositor: IUnknown,
        host: &ElementRef<Grid>,
        width: f32,
        height: f32,
    ) -> Result<Self> {
        let compositor = Compositor::from_host(compositor)?;

        let root = compositor.create_container_visual();
        let background = compositor.create_sprite_visual();
        background.set_brush(&compositor.create_color_brush(CompositionColor::rgb(24, 24, 32)));
        root.children().insert_at_bottom(&background);

        let square = compositor.create_sprite_visual();
        square.set_size(160.0, 160.0);
        square.set_brush(&compositor.create_color_brush(CompositionColor::rgb(0, 120, 215)));
        root.children().insert_at_top(&square);

        let _ = host.request_set_child_visual(Some(root.host_visual()), |result| {
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
    scale: f64,
    host: ElementRef<Grid>,
    scene: Rc<RefCell<Option<Scene>>>,
}

impl Component for Sample {
    type Input = ();
    type Message = f64;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            scale: 1.0,
            host: ElementRef::new(),
            scene: Rc::new(RefCell::new(None)),
        }
    }

    fn update(&mut self, scale: f64, _context: &ComponentContext<Self>) {
        self.scale = scale;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Composition Host");
        let host = self.host.clone();
        let scene = Rc::clone(&self.scene);
        let sender = context.sender();
        context.use_effect_guard("composition-host", (), move || {
            let event_host = host.clone();
            host.observe_composition_host(move |event| {
                let scale = match event {
                    CompositionHostEvent::Ready {
                        compositor,
                        width,
                        height,
                        scale,
                    } => {
                        match Scene::build(compositor, &event_host, width as f32, height as f32) {
                            Ok(built) => *scene.borrow_mut() = Some(built),
                            Err(error) => eprintln!("composition init failed: {error}"),
                        }
                        scale
                    }
                    CompositionHostEvent::Metrics {
                        width,
                        height,
                        scale,
                    } => {
                        if let Some(scene) = scene.borrow().as_ref() {
                            scene.layout(width as f32, height as f32).unwrap();
                        }
                        scale
                    }
                };
                sender.send(scale);
            })
        });
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                TextBlock::new()
                    .text(format!("rasterization scale: {:.2}x", self.scale))
                    .font_size(20.0)
                    .font_weight(FontWeight::BOLD)
                    .margin(Thickness::uniform(16.0))
                    .grid_row(0),
                Grid::new().element_ref(&self.host).grid_row(1),
            ))
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
