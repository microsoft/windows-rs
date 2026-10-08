#![windows_subsystem = "windows"]

use windows_reactor::*;

// XamlChangeId::IconNoGridOptimization from the Windows App SDK.
const ICON_NO_GRID_OPTIMIZATION: i32 = 61276805;

struct Example;

impl Component for Example {
    type Input = bool;
    type Message = ();

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, accepted: &bool, context: &mut ViewContext<Self>) -> View {
        let enabled = OptionalChanges::is_enabled(ICON_NO_GRID_OPTIMIZATION).unwrap();
        context.window_frame(
            "Optional WinUI changes",
            format!("Enable request accepted: {accepted}\nChange enabled after startup: {enabled}"),
        )
    }
}

fn main() {
    let accepted = OptionalChanges::enable(ICON_NO_GRID_OPTIMIZATION).unwrap();
    App::run_component::<Example>(accepted).unwrap();
}
