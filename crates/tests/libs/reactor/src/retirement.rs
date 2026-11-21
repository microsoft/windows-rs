use super::*;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub(super) struct RetirementFixture {
    target: ElementRef<Button>,
    callback: Option<Callback<()>>,
    released: Rc<Cell<bool>>,
    publication: Option<(bool, bool)>,
    scopes: Rc<ScopeLifetimes>,
    scope_publication: (u32, u32),
    swapped: bool,
    paged: bool,
    selected: u32,
    calls: u32,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Remove,
    Swap,
    Page,
    Selected(u32),
    Published(bool, bool, u32, u32),
}

struct ClickLifetime {
    sender: LocalSender<Message>,
    released: Rc<Cell<bool>>,
}

impl ClickLifetime {
    fn invoke(&self) {
        assert!(self.sender.send(Message::Selected(0)));
    }
}

impl Drop for ClickLifetime {
    fn drop(&mut self) {
        self.released.set(true);
    }
}

impl Component for RetirementFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        let released = Rc::new(Cell::new(false));
        let click = ClickLifetime {
            sender: context.sender(),
            released: Rc::clone(&released),
        };
        Self {
            target: ElementRef::new(),
            callback: Some(Callback::new(move |()| click.invoke())),
            released,
            publication: None,
            scopes: Rc::default(),
            scope_publication: (0, 0),
            swapped: false,
            paged: false,
            selected: 0,
            calls: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Remove => self.callback = None,
            Message::Swap => self.swapped = true,
            Message::Page => self.paged = !self.paged,
            Message::Selected(generation) => {
                self.selected = generation;
                self.calls += 1;
            }
            Message::Published(released, queued, cleaned, dropped) => {
                self.publication = Some((released, queued));
                self.scope_publication = (cleaned, dropped);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let target = self.target.clone();
        let released = Rc::clone(&self.released);
        let sender = context.sender();
        let scopes = Rc::clone(&self.scopes);
        context.use_effect(
            "retirement",
            (self.callback.is_none(), self.paged),
            move || {
                // Snapshot logical retirement before returning to the native event loop.
                assert!(sender.send(Message::Published(
                    released.get(),
                    target.request_focus(),
                    scopes.cleaned.get(),
                    scopes.dropped.get(),
                )));
                None
            },
        );
        let first =
            component::<TrackedChild>(2u32, ("first", "First", Rc::clone(&self.scopes))).keyed();
        let second =
            component::<TrackedChild>(3u32, ("second", "Second", Rc::clone(&self.scopes))).keyed();
        // Removal retires the child while WinUI animates it out and appends a replacement. Swap
        // then moves the remaining siblings.
        let children = if let Some(callback) = &self.callback {
            vec![
                first,
                keyed(
                    0u32,
                    Button::new()
                        .automation_id("old")
                        .element_ref(&self.target)
                        .content("Old action")
                        .on_click(callback.clone()),
                ),
                second,
            ]
        } else {
            let replacement = keyed(
                1u32,
                Button::new()
                    .automation_id("replacement")
                    .content("Replacement action")
                    .on_click(context.message(Message::Selected(1))),
            );
            if self.swapped {
                vec![second, first, replacement]
            } else {
                vec![first, second, replacement]
            }
        };
        let publication = match self.publication {
            Some((released, queued)) => {
                format!("Callback released {released}; reference queued {queued}")
            }
            None => "Awaiting publication".to_owned(),
        };
        // Paging replaces the Border's whole subtree, which WinUI animates with its content transition.
        let page: View = if self.paged {
            Grid::new()
                .children((StackPanel::new().children((
                    TextBlock::new().automation_id("page-text").text("Page B"),
                    Button::new()
                        .automation_id("page-action")
                        .content("Page action")
                        .on_click(context.message(Message::Selected(2))),
                )),))
                .into()
        } else {
            StackPanel::new()
                .children_transitions([
                    ThemeTransition::AddDelete,
                    ThemeTransition::Entrance,
                    ThemeTransition::Reposition,
                ])
                .keyed_children(children)
                .into()
        };
        context.window_title("Reactor UI fixture: retirement");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 500.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Border::new()
                    .child_transitions([ThemeTransition::Content])
                    .content(page),
                component::<TrackedChild>(
                    "retained",
                    ("retained-scope", "Retained scope", Rc::clone(&self.scopes)),
                ),
                Button::new()
                    .automation_id("remove")
                    .content("Remove child")
                    .on_click(context.message(Message::Remove)),
                Button::new()
                    .automation_id("swap")
                    .content("Swap siblings")
                    .on_click(context.message(Message::Swap)),
                Button::new()
                    .automation_id("page")
                    .content("Swap page")
                    .on_click(context.message(Message::Page)),
                TextBlock::new()
                    .automation_id("publication")
                    .text(publication),
                TextBlock::new()
                    .automation_id("scope-lifetimes")
                    .text(format!(
                        "Cleanups {}; drops {}",
                        self.scope_publication.0, self.scope_publication.1
                    )),
                TextBlock::new()
                    .automation_id("selection")
                    .text(format!("Selected {}; calls {}", self.selected, self.calls)),
            ))
            .into()
    }
}

#[derive(Default, PartialEq)]
struct ScopeLifetimes {
    cleaned: Cell<u32>,
    dropped: Cell<u32>,
}

struct TrackedChild {
    scopes: Rc<ScopeLifetimes>,
    cleaned: Rc<Cell<bool>>,
}

impl Component for TrackedChild {
    type Input = (&'static str, &'static str, Rc<ScopeLifetimes>);
    type Message = ();

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self {
            scopes: Rc::clone(&input.2),
            cleaned: Rc::default(),
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let scopes = Rc::clone(&self.scopes);
        let cleaned = Rc::clone(&self.cleaned);
        context.use_effect("lifetime", (), move || {
            Some(Box::new(move || {
                assert!(!cleaned.replace(true));
                scopes.cleaned.set(scopes.cleaned.get() + 1);
            }))
        });
        TextBlock::new().automation_id(input.0).text(input.1).into()
    }
}

impl Drop for TrackedChild {
    fn drop(&mut self) {
        assert!(
            self.cleaned.get(),
            "component dropped before effect cleanup"
        );
        self.scopes.dropped.set(self.scopes.dropped.get() + 1);
    }
}

pub(super) struct NestedFixture {
    state: Rc<NestedState>,
    shown: bool,
    mounted: usize,
    retired: usize,
    checks: usize,
    snapshot: String,
}

#[derive(Clone, Copy)]
pub(super) enum NestedMessage {
    Toggle,
    Mounted,
    Retired,
    Check,
    SendRetired,
}

impl Component for NestedFixture {
    type Input = bool;
    type Message = NestedMessage;

    fn create(_: &bool, _: &ComponentContext<Self>) -> Self {
        Self {
            state: Rc::default(),
            shown: true,
            mounted: 0,
            retired: 0,
            checks: 0,
            snapshot: String::new(),
        }
    }

    fn update(&mut self, message: NestedMessage, context: &ComponentContext<Self>) {
        match message {
            NestedMessage::Toggle => self.shown = !self.shown,
            NestedMessage::Mounted => self.mounted += 1,
            NestedMessage::Retired => self.retired += 1,
            NestedMessage::Check => {
                self.checks += 1;
                self.snapshot = match self.state.check() {
                    Ok((created, cleaned, dropped)) => {
                        format!("Created {created}; cleaned {cleaned}; dropped {dropped}")
                    }
                    Err(error) => format!("{error}: {:?}", self.state.events.borrow()),
                };
            }
            NestedMessage::SendRetired => {
                for sender in self.state.retired_senders.borrow().iter() {
                    assert!(sender.send(()));
                }
                // The check follows the stale messages in the same queue.
                assert!(context.sender().send(NestedMessage::Check));
            }
        }
    }

    fn view(&self, virtual_rows: &bool, context: &mut ViewContext<Self>) -> View {
        context.window_title(if *virtual_rows {
            "Reactor UI fixture: retirement-virtual"
        } else {
            "Reactor UI fixture: retirement-nested"
        });
        context.window_visuals(WindowVisuals::new().client_size(600.0, 650.0));
        let input = NestedInput {
            node: 0,
            parent: None,
            state: self.state.clone(),
            report: context.sender(),
        };
        let content = if *virtual_rows {
            let first = usize::from(!self.shown);
            ScrollViewer::new()
                .automation_id("rows")
                .height(220.0)
                .content(ItemsRepeater::new().virtual_source(VirtualSource::new(
                    first as u64,
                    200 - first,
                    move |index| index + first,
                    move |index| {
                        let index = index + first;
                        if index == 0 {
                            View::component::<NestedNode>(input.clone())
                        } else {
                            TextBlock::new()
                                .automation_id(format!("row-{index}"))
                                .text(format!("Row {index}"))
                                .height(100.0)
                                .into()
                        }
                    },
                )))
                .into()
        } else if self.shown {
            View::component::<NestedNode>(input)
        } else {
            TextBlock::new().text("Removed").into()
        };
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                Button::new()
                    .automation_id("toggle")
                    .content("Toggle subtree")
                    .on_click(context.message(NestedMessage::Toggle)),
                Button::new()
                    .automation_id("check")
                    .content("Check lifetimes")
                    .on_click(context.message(NestedMessage::Check)),
                Button::new()
                    .automation_id("send-retired")
                    .content("Send to retired scopes")
                    .on_click(context.message(NestedMessage::SendRetired)),
                TextBlock::new()
                    .automation_id("mounted")
                    .text(format!("Mounted {}", self.mounted)),
                TextBlock::new()
                    .automation_id("retired")
                    .text(format!("Retired {}", self.retired)),
                TextBlock::new()
                    .automation_id("checks")
                    .text(format!("Checks {}", self.checks)),
                TextBlock::new()
                    .automation_id("snapshot")
                    .text(self.snapshot.clone()),
                component::<NestedNode>(
                    "retained",
                    NestedInput {
                        node: 5,
                        parent: None,
                        state: self.state.clone(),
                        report: context.sender(),
                    },
                ),
                content,
            ))
            .into()
    }
}

#[derive(Default)]
struct NestedState {
    parents: RefCell<Vec<Option<usize>>>,
    events: RefCell<Vec<LifetimeEvent>>,
    timers: RefCell<Vec<ComponentTimer>>,
    retired_senders: RefCell<Vec<LocalSender<()>>>,
}

#[derive(Debug)]
enum LifetimeEvent {
    Setup(usize, u8),
    Cleanup(usize, u8, bool),
    Drop(usize),
    Delivered(usize),
}

impl NestedState {
    fn check(&self) -> Result<(usize, usize, usize), &'static str> {
        #[derive(Default)]
        struct Observed {
            setup: Vec<u8>,
            cleanup: Vec<(u8, usize)>,
            dropped: Option<usize>,
            delivered: bool,
        }
        let parents = self.parents.borrow();
        let mut observed: Vec<Observed> = parents.iter().map(|_| Observed::default()).collect();
        for (position, event) in self.events.borrow().iter().enumerate() {
            match *event {
                LifetimeEvent::Setup(id, effect) => observed[id].setup.push(effect),
                LifetimeEvent::Cleanup(id, effect, cancelled) => {
                    if !cancelled {
                        return Err("timer payload survived until effect cleanup");
                    }
                    observed[id].cleanup.push((effect, position));
                }
                LifetimeEvent::Drop(id) => {
                    if observed[id].dropped.replace(position).is_some() {
                        return Err("component dropped twice");
                    }
                }
                LifetimeEvent::Delivered(id) => {
                    observed[id].delivered = true;
                }
            }
        }
        for (id, node) in observed.iter().enumerate() {
            if node.delivered {
                return Err("message reached a retired scope or its replacement");
            }
            if node.setup != [0, 1] {
                return Err("effects were not set up exactly once");
            }
            if !node.cleanup.is_empty()
                && node
                    .cleanup
                    .iter()
                    .map(|(effect, _)| *effect)
                    .collect::<Vec<_>>()
                    != [1, 0]
            {
                return Err("effects were not cleaned exactly once in reverse order");
            }
            if let Some(dropped) = node.dropped
                && node
                    .cleanup
                    .last()
                    .is_none_or(|(_, cleanup)| *cleanup >= dropped)
            {
                return Err("component dropped before its cleanup");
            }
            if let Some(parent) = parents[id] {
                let parent = &observed[parent];
                if let Some((_, cleanup)) = node.cleanup.first()
                    && parent
                        .cleanup
                        .last()
                        .is_none_or(|(_, parent)| parent >= cleanup)
                {
                    return Err("child cleanup preceded parent cleanup");
                }
                if let Some(parent_drop) = parent.dropped
                    && node
                        .dropped
                        .is_none_or(|child_drop| child_drop >= parent_drop)
                {
                    return Err("parent dropped before a child");
                }
            }
        }
        Ok((
            observed.len(),
            observed
                .iter()
                .filter(|node| !node.cleanup.is_empty())
                .count(),
            observed
                .iter()
                .filter(|node| node.dropped.is_some())
                .count(),
        ))
    }
}

#[derive(Clone)]
struct NestedInput {
    node: u8,
    parent: Option<usize>,
    state: Rc<NestedState>,
    report: LocalSender<NestedMessage>,
}

impl PartialEq for NestedInput {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
            && self.parent == other.parent
            && Rc::ptr_eq(&self.state, &other.state)
    }
}

struct PendingPayload(Arc<AtomicBool>);

impl Drop for PendingPayload {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

struct NestedNode {
    input: NestedInput,
    instance: usize,
    cancelled: Arc<AtomicBool>,
    sender: LocalSender<()>,
}

impl Component for NestedNode {
    type Input = NestedInput;
    type Message = ();

    fn create(input: &NestedInput, context: &ComponentContext<Self>) -> Self {
        let instance = input.state.parents.borrow().len();
        input.state.parents.borrow_mut().push(input.parent);
        let cancelled = Arc::new(AtomicBool::new(false));
        let payload = PendingPayload(cancelled.clone());
        let timer = context.set_local_timeout(Duration::from_secs(3_600), move || {
            drop(payload);
            panic!("pending retirement timer fired");
        });
        // The surviving fixture retains every timer across subtree retirement.
        input.state.timers.borrow_mut().push(timer);
        Self {
            input: input.clone(),
            instance,
            cancelled,
            sender: context.sender(),
        }
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.input
            .state
            .events
            .borrow_mut()
            .push(LifetimeEvent::Delivered(self.instance));
    }

    fn view(&self, _: &NestedInput, context: &mut ViewContext<Self>) -> View {
        for effect in 0..2 {
            let state = self.input.state.clone();
            let instance = self.instance;
            let cancelled = self.cancelled.clone();
            let report = self.input.report.clone();
            let node = self.input.node;
            context.use_effect(effect as u32, (), move || {
                state
                    .events
                    .borrow_mut()
                    .push(LifetimeEvent::Setup(instance, effect));
                if node == 0 && effect == 1 {
                    assert!(report.send(NestedMessage::Mounted));
                }
                Some(Box::new(move || {
                    state.events.borrow_mut().push(LifetimeEvent::Cleanup(
                        instance,
                        effect,
                        cancelled.load(Ordering::SeqCst),
                    ));
                }))
            });
        }
        let children: &[u8] = match self.input.node {
            0 => &[1, 4],
            1 => &[2, 3],
            _ => &[],
        };
        let mut views = vec![
            TextBlock::new()
                .automation_id(format!("nested-{}", self.input.node))
                .text(format!("Node {}", self.input.node))
                .height(20.0)
                .into(),
        ];
        views.extend(children.iter().map(|&node| {
            View::component::<Self>(NestedInput {
                node,
                parent: Some(self.instance),
                ..self.input.clone()
            })
        }));
        StackPanel::new().children(views).into()
    }
}

impl Drop for NestedNode {
    fn drop(&mut self) {
        self.input
            .state
            .events
            .borrow_mut()
            .push(LifetimeEvent::Drop(self.instance));
        self.input
            .state
            .retired_senders
            .borrow_mut()
            .push(self.sender.clone());
        if self.input.node == 0 {
            _ = self.input.report.send(NestedMessage::Retired);
        }
    }
}
