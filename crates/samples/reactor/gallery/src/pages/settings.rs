use windows_reactor::*;

pub struct SettingsPage;

impl Component for SettingsPage {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &(), _context: &mut ViewContext<Self>) -> View {
        ScrollViewer::new().content(
            Border::new()
                .padding(Thickness::new(36.0, 24.0, 36.0, 36.0))
                .content(
                    StackPanel::new().spacing(24.0).children((
                        TextBlock::new()
                            .text("Settings")
                            .font_size(28.0)
                            .font_weight(FontWeight::BOLD),
                        Border::new().padding(20.0).content(
                            StackPanel::new().spacing(12.0).children((
                                TextBlock::new()
                                    .text("About this app")
                                    .font_weight(FontWeight::BOLD),
                                StackPanel::new().spacing(2.0).children((
                                    TextBlock::new()
                                        .text("WinUI Gallery (Reactor)")
                                        .font_weight(FontWeight::BOLD),
                                    TextBlock::new()
                                        .text("Version 0.1.0")
                                        .font_size(12.0)
                                        .opacity(0.6),
                                )),
                                TextBlock::new()
                                    .text(
                                        "This app is built with Reactor, a declarative \
                                     component-based UI framework for WinUI 3. It recreates the \
                                     WinUI Gallery experience using the Reactor component \
                                     model and a generated, typed element tree.",
                                    )
                                    .font_size(13.0)
                                    .opacity(0.6),
                            )),
                        ),
                        Border::new().padding(20.0).content(
                            StackPanel::new().spacing(8.0).children((
                                TextBlock::new()
                                    .text("Built with Reactor")
                                    .font_weight(FontWeight::BOLD),
                                TextBlock::new()
                                    .text("Framework: Reactor (typed Rust component model)")
                                    .font_size(13.0),
                                TextBlock::new()
                                    .text("Platform: WinUI 3 / Windows App SDK")
                                    .font_size(13.0),
                                TextBlock::new()
                                    .text("Rendering: generated element tree reconciler")
                                    .font_size(13.0),
                                TextBlock::new()
                                    .text("State: component-owned Message/update state machine")
                                    .font_size(13.0),
                            )),
                        ),
                    )),
                ),
        )
    }
}
