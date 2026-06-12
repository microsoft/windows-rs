use windows_reactor::*;

pub fn run(title: &str, render: fn(&mut RenderCx) -> Element) -> Result<()> {
    let _bootstrap_handle = bootstrap::initialize()?;
    App::new().title(title).render(render)
}
