use windows_reactor::*;

pub fn run(title: &str, render: fn(&mut RenderCx) -> Element) -> Result<()> {
    bootstrap()?;
    App::new().title(title).render(render)
}
