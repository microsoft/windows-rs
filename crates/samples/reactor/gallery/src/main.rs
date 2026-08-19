#![windows_subsystem = "windows"]

mod registry;
mod router;
mod shell;

fn main() -> windows_core::Result<()> {
    windows_reactor::bootstrap()?;
    windows_reactor::run_reactor_winui_app(windows_reactor::component(shell::gallery))
}
