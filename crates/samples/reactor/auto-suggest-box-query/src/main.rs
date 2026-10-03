use std::rc::Rc;

use windows_reactor::*;

struct Search {
    submitted: Rc<str>,
}

impl Component for Search {
    type Input = ();
    type Message = Rc<str>;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            submitted: "(none)".into(),
        }
    }

    fn update(&mut self, submitted: Rc<str>, _: &ComponentContext<Self>) {
        self.submitted = submitted;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .spacing(8.0)
            .children((
                AutoSuggestBox::new()
                    .placeholder_text("Search...")
                    .query_icon(Symbol::Find)
                    .on_query_submitted(context.forward()),
                TextBlock::new().text(format!("Submitted: {}", self.submitted)),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<Search>(()).unwrap();
}
