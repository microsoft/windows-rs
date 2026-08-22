const TASKS: usize = 32;

#[derive(Clone, PartialEq)]
pub struct Task {
    pub done: bool,
    pub id: u64,
    pub key: String,
    pub title: String,
}

#[derive(Clone, PartialEq)]
pub struct Model {
    pub query: String,
    pub selected: u64,
    pub tasks: Vec<Task>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            query: "ready".to_string(),
            selected: 0,
            tasks: (0..TASKS)
                .map(|index| Task {
                    done: index.is_multiple_of(3),
                    id: index as u64,
                    key: index.to_string(),
                    title: format!("Task {index:02}"),
                })
                .collect(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Operation {
    BroadToggle,
    LocalEdit,
    Reverse,
    Selection,
    ValueEqual,
}

pub const OPERATIONS: [(&str, Operation); 5] = [
    ("local edit", Operation::LocalEdit),
    ("selection", Operation::Selection),
    ("broad toggle", Operation::BroadToggle),
    ("reverse keys", Operation::Reverse),
    ("value equal", Operation::ValueEqual),
];

impl Model {
    pub fn apply(&mut self, operation: Operation, iteration: usize) {
        match operation {
            Operation::BroadToggle => {
                for task in &mut self.tasks {
                    task.done = !task.done;
                }
            }
            Operation::LocalEdit => {
                let index = iteration % self.tasks.len();
                let task = &mut self.tasks[index];
                task.title = if task.title.ends_with('*') {
                    format!("Task {:02}", task.id)
                } else {
                    format!("Task {:02}*", task.id)
                };
            }
            Operation::Reverse => self.tasks.reverse(),
            Operation::Selection => self.selected = if self.selected == 0 { 1 } else { 0 },
            Operation::ValueEqual => {}
        }
    }
}

#[cfg(feature = "next")]
pub mod next {
    use super::Model;
    use windows_reactor_next::*;

    pub fn view(model: &Model) -> View {
        let pane = StackPanel::new().spacing(6.0).children((
            TextBlock::new().text("Tasks"),
            TextBlock::new().text("Dashboard"),
            TextBlock::new().text("Active"),
            TextBlock::new().text("Settings"),
        ));
        let rows = model.tasks.iter().map(|task| {
            KeyedView::new(
                task.key.clone(),
                Grid::new()
                    .column_spacing(8.0)
                    .columns([
                        GridLength::STAR,
                        GridLength::Pixel(80.0),
                        GridLength::Pixel(80.0),
                    ])
                    .children((
                        TextBlock::new().text(task.title.clone()),
                        ToggleSwitch::new().is_on(task.done).grid_column(1),
                        TextBlock::new()
                            .text(if model.selected == task.id {
                                "selected"
                            } else {
                                "select"
                            })
                            .grid_column(2),
                    )),
            )
        });
        let content = Grid::new()
            .row_spacing(8.0)
            .column_spacing(12.0)
            .rows([
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::STAR,
            ])
            .columns([GridLength::Pixel(120.0), GridLength::STAR])
            .children((
                TextBlock::new().text("Task workspace").grid_column_span(2),
                TextBlock::new().text("Filter").grid_row(1),
                TextBox::new()
                    .text(model.query.clone())
                    .grid_row(1)
                    .grid_column(1),
                TextBlock::new()
                    .text(format!(
                        "{} tasks; selected {}",
                        model.tasks.len(),
                        model.selected
                    ))
                    .grid_row(2)
                    .grid_column_span(2),
                StackPanel::new()
                    .spacing(4.0)
                    .grid_row(3)
                    .grid_column_span(2)
                    .keyed_children(rows),
            ));
        SplitView::new()
            .open_pane_length(200.0)
            .display_mode(SplitViewDisplayMode::Inline)
            .is_pane_open(true)
            .slots([
                SlotView::new(SplitViewSlot::Pane, pane),
                SlotView::new(SplitViewSlot::Content, content),
            ])
    }
}

#[cfg(feature = "incumbent")]
pub mod incumbent {
    use super::Model;
    use windows_reactor::*;

    pub fn view(model: &Model) -> Element {
        let pane = vstack((
            text_block("Tasks"),
            text_block("Dashboard"),
            text_block("Active"),
            text_block("Settings"),
        ))
        .spacing(6.0);
        let rows = model
            .tasks
            .iter()
            .map(|task| {
                grid((
                    text_block(task.title.clone()),
                    ToggleSwitch::new(task.done).grid_column(1),
                    text_block(if model.selected == task.id {
                        "selected"
                    } else {
                        "select"
                    })
                    .grid_column(2),
                ))
                .columns([
                    GridLength::Star(1.0),
                    GridLength::Pixel(80.0),
                    GridLength::Pixel(80.0),
                ])
                .column_spacing(8.0)
                .with_key(task.key.clone())
                .into()
            })
            .collect::<Vec<_>>();
        let content = grid((
            text_block("Task workspace").grid_column_span(2),
            text_block("Filter").grid_row(1),
            text_box(model.query.clone()).grid_row(1).grid_column(1),
            text_block(format!(
                "{} tasks; selected {}",
                model.tasks.len(),
                model.selected
            ))
            .grid_row(2)
            .grid_column_span(2),
            vstack(rows).spacing(4.0).grid_row(3).grid_column_span(2),
        ))
        .rows([
            GridLength::Auto,
            GridLength::Auto,
            GridLength::Auto,
            GridLength::Star(1.0),
        ])
        .columns([GridLength::Pixel(120.0), GridLength::Star(1.0)])
        .row_spacing(8.0)
        .column_spacing(12.0);
        split_view(content)
            .pane(pane)
            .open_pane_length(200.0)
            .display_mode(SplitViewDisplayMode::Inline)
            .is_pane_open(true)
            .into()
    }
}
