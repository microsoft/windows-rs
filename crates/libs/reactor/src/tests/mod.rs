use super::*;

mod attachments;
mod declarations;
mod dialogs;
mod events;
mod failures;
mod feedback;
mod input;
mod reconcile;
mod reference;
mod retirement;
mod selection;
mod virtualization;

fn text(value: &str) -> View {
    TextBlock::new().text(value).into()
}
