use super::*;

pub(super) struct BackdropFixture {
    material: Option<SystemBackdrop>,
    radius: Option<f64>,
    resized: bool,
    attached: bool,
    clicks: u32,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Material(Option<SystemBackdrop>),
    Radius(Option<f64>),
    Resize,
    Attach,
    Click,
}

impl Component for BackdropFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            material: Some(SystemBackdrop::Acrylic),
            radius: Some(32.0),
            resized: false,
            attached: true,
            clicks: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Material(value) => self.material = value,
            Message::Radius(value) => self.radius = value,
            Message::Resize => self.resized = !self.resized,
            Message::Attach => self.attached = !self.attached,
            Message::Click => self.clicks += 1,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: backdrop");
        context.window_visuals(
            WindowVisuals::new()
                .client_size(760.0, 420.0)
                .theme(WindowTheme::Light),
        );
        let action = |id: &str, message| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(message))
        };
        let mut effect = SystemBackdropElement::new()
            .width(if self.resized { 280.0 } else { 200.0 })
            .height(if self.resized { 160.0 } else { 120.0 })
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Top);
        if let Some(material) = self.material {
            effect = effect.system_backdrop(material);
        }
        if let Some(radius) = self.radius {
            effect = effect.corner_radius(radius);
        }
        let mut children = Vec::new();
        if self.attached {
            children.push(keyed("effect", effect));
        }
        children.push(keyed(
            "foreground",
            Button::new()
                .automation_id("foreground")
                .content("Click")
                .width(80.0)
                .height(32.0)
                .margin(Thickness::new(60.0, 60.0, 0.0, 0.0))
                .horizontal_alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Top)
                .on_click(context.message(Message::Click)),
        ));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        action("acrylic", Message::Material(Some(SystemBackdrop::Acrylic))),
                        action("mica", Message::Material(Some(SystemBackdrop::Mica))),
                        action("mica-alt", Message::Material(Some(SystemBackdrop::MicaAlt))),
                        action("none", Message::Material(Some(SystemBackdrop::None))),
                        action("omit", Message::Material(None)),
                    )),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        action("round", Message::Radius(Some(32.0))),
                        action("square", Message::Radius(Some(0.0))),
                        action("clear-corners", Message::Radius(None)),
                        action("resize", Message::Resize),
                        action("attach", Message::Attach),
                    )),
                Border::new()
                    .automation_id("bounds")
                    .automation_name("Backdrop bounds")
                    .is_tab_stop(true)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .width(320.0)
                    .height(180.0)
                    .background(Color::rgb(255, 0, 0))
                    .content(Grid::new().keyed_children(children)),
                TextBlock::new()
                    .automation_id("clicks")
                    .text(self.clicks.to_string()),
            ))
            .into()
    }
}
