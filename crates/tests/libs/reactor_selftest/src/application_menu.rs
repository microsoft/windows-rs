use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use windows_reactor::{
    App, AppCallback, Component, ComponentContext, Menu, MenuItem, ScreenPoint, TextBlock, View,
    ViewContext,
};

#[derive(Clone)]
struct CloseSignal(Rc<AppCallback>);

impl PartialEq for CloseSignal {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

struct ClosingWindow(CloseSignal);

impl Component for ClosingWindow {
    type Input = CloseSignal;
    type Message = ();

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        assert!(context.close_window());
        Self(input.clone())
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("Closing").into()
    }
}

impl Drop for ClosingWindow {
    fn drop(&mut self) {
        self.0.0.invoke().unwrap();
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        eprintln!("Reactor application menu fixture timed out");
        std::process::exit(1);
    });
    let completed = Rc::new(Cell::new(false));
    let run_completed = Rc::clone(&completed);
    App::run_with(move |app| {
        app.show_menu_at(
            ScreenPoint::new(32, 32),
            Menu::new(
                [
                    MenuItem::item("open", "Open"),
                    MenuItem::separator("separator"),
                    MenuItem::submenu("more", "More", [MenuItem::item("exit", "Exit")]),
                ],
                |_| {},
            ),
        )?;

        let final_completed = Rc::clone(&run_completed);
        let final_context = app.clone();
        let window_closed = app.callback(move || {
            assert!(
                final_context
                    .show_menu_at(
                        ScreenPoint::new(32, 32),
                        Menu::new([MenuItem::item("duplicate", "Duplicate")], |_| {}),
                    )
                    .is_err()
            );
            final_completed.set(true);
            final_context.exit()
        });
        app.open_component_window::<ClosingWindow>(CloseSignal(Rc::new(window_closed)))?;
        Ok(())
    })?;
    assert!(completed.get());
    Ok(())
}
