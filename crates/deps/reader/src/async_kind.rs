#[derive(PartialEq)]
pub enum AsyncKind {
    None,
    Action,
    ActionWithProgress,
    Operation,
    OperationWithProgress,
}
