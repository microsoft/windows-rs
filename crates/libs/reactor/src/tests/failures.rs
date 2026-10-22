use super::*;

struct NeverCalledAdapter;

impl Adapter for NeverCalledAdapter {
    fn apply(&mut self, _mutations: &[Mutation]) {
        unreachable!("invalid declarations must not reach the adapter")
    }
}

#[derive(Default)]
struct ApplyOnceAdapter {
    applied: bool,
}

impl Adapter for ApplyOnceAdapter {
    fn apply(&mut self, _mutations: &[Mutation]) {
        assert!(!self.applied, "invalid updates must not reach the adapter");
        self.applied = true;
    }
}

#[test]
#[should_panic(expected = "declaration exceeds the maximum depth")]
fn excessive_depth_is_rejected_before_reconciliation() {
    let mut child: View = TextBlock::new().text("leaf").into();
    for _ in 0..10_000 {
        child = Border::new().content(child).into();
    }
    Runtime::new(NeverCalledAdapter).update(child);
}

#[test]
#[should_panic(expected = "declaration exceeds the maximum of")]
fn shared_declarations_cannot_expand_beyond_the_graph_limit() {
    let mut child: View = TextBlock::new().text("leaf").into();
    for _ in 0..17 {
        child = StackPanel::new().children([child.clone(), child]).into();
    }
    Runtime::new(NeverCalledAdapter).update(child);
}

#[test]
#[should_panic(expected = "root type changed from Border to Grid")]
fn root_type_changes_are_rejected_before_native_mutation() {
    let mut runtime = Runtime::new(ApplyOnceAdapter::default());
    runtime.update(Border::new());
    runtime.update(Grid::new());
}
