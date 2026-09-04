#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_composition::{CompositionColor, Compositor, SpriteVisual};
use windows_core::Result;
use windows_reactor::*;

fn build(compositor: windows_core::IUnknown, host: &ElementRef<Grid>) -> Result<SpriteVisual> {
    let compositor = Compositor::from_host(compositor)?;
    let visual = compositor.create_sprite_visual();
    visual.set_brush(&compositor.create_color_brush(CompositionColor::rgb(96, 64, 160)));
    let _ = host.request_set_child_visual(Some(visual.as_raw().into()), |_| {});
    Ok(visual)
}

struct Sample {
    scale: f64,
    host: ElementRef<Grid>,
    visual: Rc<RefCell<Option<SpriteVisual>>>,
}

impl Component for Sample {
    type Input = ();
    type Message = f64;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            scale: 1.0,
            host: ElementRef::new(),
            visual: Rc::new(RefCell::new(None)),
        }
    }

    fn update(&mut self, scale: f64, _context: &ComponentContext<Self>) {
        self.scale = scale;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Composition DPI");
        let host = self.host.clone();
        let visual = Rc::clone(&self.visual);
        let sender = context.sender();
        context.use_effect("composition-host", (), move || {
            let event_host = host.clone();
            let observation = host.observe_composition_host(move |event| {
                let (width, height, scale) = match event {
                    CompositionHostEvent::Ready {
                        compositor,
                        width,
                        height,
                        scale,
                    } => {
                        match build(compositor, &event_host) {
                            Ok(built) => *visual.borrow_mut() = Some(built),
                            Err(error) => eprintln!("composition init failed: {error}"),
                        }
                        (width, height, scale)
                    }
                    CompositionHostEvent::Metrics {
                        width,
                        height,
                        scale,
                    } => (width, height, scale),
                };
                if let Some(visual) = visual.borrow().as_ref() {
                    visual.set_size(width as f32, height as f32);
                }
                sender.send(scale);
            });
            Some(Box::new(move || drop(observation)))
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
