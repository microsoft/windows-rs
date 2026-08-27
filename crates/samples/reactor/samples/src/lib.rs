use windows_core::Result;

#[derive(Clone)]
struct SampleInput {
    render: std::rc::Rc<dyn Fn() -> windows_reactor::View>,
    title: &'static str,
}

impl PartialEq for SampleInput {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && std::rc::Rc::ptr_eq(&self.render, &other.render)
    }
}

struct Sample(SampleInput);

impl windows_reactor::Component for Sample {
    type Message = ();
    type Input = SampleInput;

    fn create(input: &Self::Input, _context: &windows_reactor::ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn input_changed(
        &mut self,
        input: &Self::Input,
        _context: &windows_reactor::ComponentContext<Self>,
    ) {
        self.0 = input.clone();
    }

    fn update(&mut self, _message: (), _context: &windows_reactor::ComponentContext<Self>) {}

    fn view(
        &self,
        _input: &Self::Input,
        context: &mut windows_reactor::ViewContext<Self>,
    ) -> windows_reactor::View {
        context.window_title(self.0.title);
        (self.0.render)()
    }
}

pub fn run(
    title: &'static str,
    render: impl Fn() -> windows_reactor::View + 'static,
) -> Result<()> {
    windows_reactor::bootstrap()?;
    windows_reactor::App::run_component::<Sample>(SampleInput {
        render: std::rc::Rc::new(render),
        title,
    })
}
