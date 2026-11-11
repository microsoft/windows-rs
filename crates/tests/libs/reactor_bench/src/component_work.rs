use super::{BenchComponent, NullAdapter, allocator, reactor};
use reactor::test::{ComponentHost, UpdateWork, measure_work};
use std::rc::Rc;
use std::sync::atomic::Ordering;

struct LargePanel(bool);

fn panel(count: usize, active: bool) -> reactor::View {
    reactor::Grid::new()
        .keyed_children((0..count).map(|index| {
            reactor::keyed(
                index,
                reactor::TextBlock::new().text(if index == 0 && active { "on" } else { "off" }),
            )
        }))
        .into()
}

impl reactor::Component for LargePanel {
    type Input = usize;
    type Message = bool;

    fn create(_: &Self::Input, _: &reactor::ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, toggle: bool, _: &reactor::ComponentContext<Self>) {
        if toggle {
            self.0 = !self.0;
        }
    }

    fn view(&self, count: &usize, _: &mut reactor::ViewContext<Self>) -> reactor::View {
        panel(*count, self.0)
    }
}

struct CachedPanel;

impl reactor::Component for CachedPanel {
    type Input = usize;
    type Message = ();

    fn create(_: &Self::Input, _: &reactor::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, count: &usize, _: &mut reactor::ViewContext<Self>) -> reactor::View {
        reactor::Border::new()
            .content(reactor::component::<LargePanel>("panel", *count))
            .into()
    }
}

struct SplitPanel;

impl reactor::Component for SplitPanel {
    type Input = usize;
    type Message = ();

    fn create(_: &Self::Input, _: &reactor::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, count: &usize, _: &mut reactor::ViewContext<Self>) -> reactor::View {
        reactor::Grid::new()
            .keyed_children((0..*count).map(|index| {
                if index == 0 {
                    reactor::component::<BenchComponent>(index, false).keyed()
                } else {
                    reactor::keyed(index, reactor::TextBlock::new().text("off"))
                }
            }))
            .into()
    }
}

struct ComponentPanel;

impl reactor::Component for ComponentPanel {
    type Input = (usize, bool);
    type Message = ();

    fn create(_: &Self::Input, _: &reactor::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(
        &self,
        (count, tooltips): &Self::Input,
        _: &mut reactor::ViewContext<Self>,
    ) -> reactor::View {
        reactor::Grid::new()
            .keyed_children((0..*count).map(|index| {
                let child = reactor::component::<BenchComponent>(index, false);
                let view: reactor::View = if *tooltips {
                    use reactor::TooltipExt;
                    reactor::Button::new().tooltip_with(reactor::Tooltip::rich(child))
                } else {
                    child.into()
                };
                reactor::keyed(index, view)
            }))
            .into()
    }
}

struct RootTooltip;

impl reactor::Component for RootTooltip {
    type Input = ();
    type Message = ();

    fn create(_: &Self::Input, _: &reactor::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Self::Input, _: &mut reactor::ViewContext<Self>) -> reactor::View {
        use reactor::TooltipExt;
        reactor::Button::new().tooltip_with(reactor::Tooltip::rich(reactor::component::<
            BenchComponent,
        >("tooltip", false)))
    }
}

#[derive(Clone)]
struct ContextInput {
    count: usize,
    context: Rc<reactor::Context<(bool, bool)>>,
}

impl PartialEq for ContextInput {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && Rc::ptr_eq(&self.context, &other.context)
    }
}

struct ContextPanel;

impl reactor::Component for ContextPanel {
    type Input = ContextInput;
    type Message = ();

    fn create(_: &Self::Input, _: &reactor::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut reactor::ViewContext<Self>) -> reactor::View {
        panel(input.count, context.use_context(&input.context).0)
    }
}

struct ContextBridge;

impl reactor::Component for ContextBridge {
    type Input = ContextInput;
    type Message = ();

    fn create(_: &Self::Input, _: &reactor::ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _: &mut reactor::ViewContext<Self>) -> reactor::View {
        reactor::Border::new()
            .content(reactor::component::<ContextPanel>("panel", input.clone()))
            .into()
    }
}

#[derive(Clone, Copy)]
enum ContextChange {
    Used,
    Unused,
}

struct ContextRoot((bool, bool));

impl reactor::Component for ContextRoot {
    type Input = ContextInput;
    type Message = ContextChange;

    fn create(_: &Self::Input, _: &reactor::ComponentContext<Self>) -> Self {
        Self((false, false))
    }

    fn update(&mut self, change: ContextChange, _: &reactor::ComponentContext<Self>) {
        match change {
            ContextChange::Used => self.0.0 = !self.0.0,
            ContextChange::Unused => self.0.1 = !self.0.1,
        }
    }

    fn view(&self, input: &Self::Input, _: &mut reactor::ViewContext<Self>) -> reactor::View {
        reactor::provide(
            &input.context,
            self.0,
            reactor::component::<ContextBridge>("bridge", input.clone()),
        )
    }
}

struct Row {
    name: &'static str,
    work: UpdateWork,
    mutations: usize,
    bytes: u64,
    allocations: u64,
}

fn measure(name: &'static str, mut operation: impl FnMut() -> usize) -> Row {
    operation();
    let bytes = allocator::allocated_bytes();
    let allocations = allocator::ALLOCATIONS.load(Ordering::Relaxed);
    let (mutations, work) = measure_work(operation);
    Row {
        name,
        work,
        mutations,
        bytes: allocator::allocated_bytes() - bytes,
        allocations: allocator::ALLOCATIONS.load(Ordering::Relaxed) - allocations,
    }
}

fn virtual_list(count: usize, revision: u64, reversed: bool, offset: usize) -> reactor::View {
    reactor::ItemsRepeater::new()
        .virtual_source(reactor::VirtualSource::new(
            revision,
            count,
            move |index| offset + if reversed { count - index - 1 } else { index },
            |_| reactor::TextBlock::new().text("row"),
        ))
        .into()
}

fn rows(count: usize) -> Vec<Row> {
    let mut rows = Vec::new();
    let mut host = ComponentHost::mount(
        NullAdapter,
        [reactor::component::<LargePanel>("root", count)],
    );
    let root = reactor::Key::from("root");
    let sender = host.sender::<LargePanel>(&root).unwrap();
    for (name, toggle) in [("large_noop", false), ("large_one_label", true)] {
        rows.push(measure(name, || {
            assert!(sender.send(toggle));
            let report = host.drain(1);
            assert_eq!(report.dispatched, 1);
            report.mutations
        }));
    }
    rows.push(measure("equal_input", || {
        host.update_input::<LargePanel>(&root, count).len()
    }));
    drop(host);

    let mut host = ComponentHost::mount(
        NullAdapter,
        [reactor::component::<CachedPanel>("root", count)],
    );
    let sender = host.sender::<CachedPanel>(&"root".into()).unwrap();
    rows.push(measure("cached_child_noop", || {
        assert!(sender.send(()));
        host.drain(1).mutations
    }));
    let child = host
        .sender_at::<LargePanel>(&["root".into(), "panel".into()])
        .unwrap();
    assert!(child.send(true));
    assert_eq!(host.drain(1).mutations, 1);
    rows.push(measure("cached_child_after_edit", || {
        assert!(sender.send(()));
        host.drain(1).mutations
    }));
    drop(host);

    let mut host = ComponentHost::mount(
        NullAdapter,
        [reactor::component::<SplitPanel>("root", count)],
    );
    let sender = host
        .sender_at::<BenchComponent>(&["root".into(), 0usize.into()])
        .unwrap();
    rows.push(measure("small_child_one_label", || {
        assert!(sender.send(true));
        host.drain(1).mutations
    }));
    drop(host);

    for (name, tooltips) in [("component_children", false), ("component_tooltips", true)] {
        let mut host = ComponentHost::mount(
            NullAdapter,
            [reactor::component::<ComponentPanel>(
                "root",
                (count, tooltips),
            )],
        );
        let sender = host.sender::<ComponentPanel>(&"root".into()).unwrap();
        rows.push(measure(name, || {
            assert!(sender.send(()));
            host.drain(1).mutations
        }));
        if !tooltips {
            let sender = host
                .sender_at::<BenchComponent>(&["root".into(), (count / 2).into()])
                .unwrap();
            rows.push(measure("leaf_among_components", || {
                assert!(sender.send(true));
                host.drain(1).mutations
            }));
        }
    }

    let mut host =
        ComponentHost::mount(NullAdapter, [reactor::component::<RootTooltip>("root", ())]);
    let sender = host.sender::<RootTooltip>(&root).unwrap();
    rows.push(measure("root_component_tooltip", || {
        assert!(sender.send(()));
        host.drain(1).mutations
    }));
    drop(host);

    let mut host = ComponentHost::mount(
        NullAdapter,
        [reactor::component::<ContextRoot>(
            "root",
            ContextInput {
                count,
                context: Rc::new(reactor::Context::new((false, false))),
            },
        )],
    );
    let sender = host.sender::<ContextRoot>(&"root".into()).unwrap();
    for (name, change) in [
        ("context_unused_field", ContextChange::Unused),
        ("context_used_field", ContextChange::Used),
    ] {
        rows.push(measure(name, || {
            assert!(sender.send(change));
            host.drain(1).mutations
        }));
    }
    drop(host);

    rows.push(measure("virtual_mount", || {
        reactor::test::Runtime::new(NullAdapter)
            .update(virtual_list(count, 0, false, 0))
            .len()
    }));
    for name in [
        "virtual_same_revision",
        "virtual_same_keys",
        "virtual_reverse",
        "virtual_churn",
        "virtual_resize",
    ] {
        let mut runtime = reactor::test::Runtime::new(NullAdapter);
        runtime.update(virtual_list(count, 0, false, 0));
        let mut revision = 0;
        rows.push(measure(name, || {
            revision += 1;
            let view = match name {
                "virtual_same_revision" => virtual_list(count, 0, false, 0),
                "virtual_same_keys" => virtual_list(count, revision, false, 0),
                "virtual_reverse" => virtual_list(count, revision, revision % 2 != 0, 0),
                "virtual_churn" => virtual_list(count, revision, false, revision as usize),
                "virtual_resize" => {
                    virtual_list(count / (1 + revision as usize % 2), revision, false, 0)
                }
                _ => unreachable!(),
            };
            runtime.update(view).len()
        }));
    }
    rows
}

pub(super) fn run(count: usize) {
    println!("component update work: count={count}, one warmed update per row (no timings)");
    println!(
        "{:<24} {:>6} {:>8} {:>9} {:>8} {:>9} {:>9} {:>9} {:>10} {:>8}",
        "workload",
        "views",
        "expand",
        "validate",
        "compare",
        "reconcile",
        "refresh",
        "mutations",
        "bytes",
        "allocs"
    );
    let rows = rows(count);
    for row in &rows {
        let work = row.work;
        println!(
            "{:<24} {:>6} {:>8} {:>9} {:>8} {:>9} {:>9} {:>9} {:>10} {:>8}",
            row.name,
            work.views,
            work.expanded,
            work.validated,
            work.compared,
            work.reconciled,
            work.refreshed,
            row.mutations,
            row.bytes,
            row.allocations
        );
    }
    println!("\nwindow publication discovery");
    println!(
        "{:<24} {:>12} {:>16}",
        "workload", "scope visits", "pending visits"
    );
    for row in &rows {
        println!(
            "{:<24} {:>12} {:>16}",
            row.name, row.work.window_scopes, row.work.window_pending,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workloads_exercise_update_boundaries() {
        for count in [1, 32] {
            let rows = rows(count);
            let find = |name| rows.iter().find(|row| row.name == name).unwrap();
            for name in [
                "large_noop",
                "cached_child_noop",
                "cached_child_after_edit",
                "component_children",
                "component_tooltips",
                "root_component_tooltip",
            ] {
                let row = find(name);
                assert_eq!(row.work.views, 1, "{name}");
                assert_eq!(row.mutations, 0, "{name}");
            }
            assert_eq!(find("equal_input").work, UpdateWork::default());
            assert_eq!(find("equal_input").mutations, 0);
            for name in [
                "large_one_label",
                "small_child_one_label",
                "leaf_among_components",
            ] {
                let row = find(name);
                assert_eq!(row.work.views, 1, "{name}");
                assert_eq!(row.mutations, 1, "{name}");
            }
            let leaf = find("small_child_one_label");
            assert_eq!(leaf.work.expanded, 1);
            assert_eq!(leaf.work.validated, 1);
            assert_eq!(leaf.work.compared, 1);
            assert_eq!(leaf.work.reconciled, 1);
            assert_eq!(leaf.work.refreshed, 1);
            for name in ["context_unused_field", "context_used_field"] {
                assert_eq!(find(name).work.views, 2, "{name}");
            }
            assert_eq!(find("context_unused_field").mutations, 0);
            assert_eq!(find("context_used_field").mutations, 1);
            assert_eq!(find("virtual_same_revision").mutations, 0);
            assert_eq!(find("virtual_same_keys").mutations, 0);
            assert_eq!(find("virtual_reverse").mutations, usize::from(count > 1));
            assert_eq!(find("virtual_churn").mutations, 1);
            assert_eq!(find("virtual_resize").mutations, 1);
        }
    }
}
