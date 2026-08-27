#![windows_subsystem = "windows"]

mod controls;
mod pages;
mod registry;
mod router;
mod shell;

#[cfg(test)]
mod tests;

fn main() {
    windows_reactor::App::run_component::<shell::Gallery>(()).unwrap();
}
