use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    grid([TextBlock::new("Sample")]).background(Color::rgb(255, 0, 0))
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
