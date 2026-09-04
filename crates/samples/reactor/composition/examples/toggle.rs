#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_composition::{CompositionColor, Compositor, SpriteVisual};
use windows_core::Result;
use windows_reactor::*;

fn build(compositor: windows_core::IUnknown, host: &ElementRef<Grid>) -> Result<SpriteVisual> {
    let compositor = Compositor::from_host(compositor)?;
    let visual = compositor.create_sprite_visual();
    visual.set_brush(&compositor.create_color_brush(CompositionColor::rgb(0, 153, 102)));
    let _ = host.request_set_child_visual(Some(visual.as_raw().into()), |_| {});
    Ok(visual)
}

struct Sample {
    shown: bool,
    host: ElementRef<Grid>,
    visual: Rc<RefCell<Option<SpriteVisual>>>,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            shown: true,
            host: ElementRef::new(),
            visual: Rc::new(RefCell::new(None)),
        }
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.shown = !self.shown;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Composition Toggle");
        let visual = Rc::clone(&self.visual);
        let shown = self.shown;
        context.use_effect("visibility", shown, move || {
            if let Some(visual) = visual.borrow().as_ref() {
                visual.set_visible(shown);
            }
            None
        });

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
                        built.set_size(width as f32, height as f32);
                        built.set_visible(shown);
                        *visual.borrow_mut() = Some(built);
                    }
                    Err(error) => eprintln!("composition init failed: {error}"),
                },
                CompositionHostEvent::Metrics { width, height, .. } => {
                    if let Some(visual) = visual.borrow().as_ref() {
                        visual.set_size(width as f32, height as f32);
                    }
                }
            });
            Some(Box::new(move || drop(observation)))
        });

        Grid::new()
            .rows([GridLength::STAR, GridLength::Auto])
            .children((
                Grid::new().element_ref(&self.host).grid_row(0),
                Button::new()
                    .on_click(context.forward())
                    .grid_row(1)
                    .margin(Thickness::uniform(16.0))
                    .content(if shown { "Hide visual" } else { "Show visual" }),
            ))
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
