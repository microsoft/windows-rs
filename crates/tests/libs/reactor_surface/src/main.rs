#![windows_subsystem = "console"]

use windows_core::Result;
use windows_reactor::App;

mod generated_surface;
mod runner;

fn main() -> Result<()> {
    App::run_component::<runner::SurfaceRunner>(runner::SurfaceConfig::from_args())
}
