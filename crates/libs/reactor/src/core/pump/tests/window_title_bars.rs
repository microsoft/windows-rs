use super::super::*;

#[derive(Clone, Copy, PartialEq)]
struct TitleBarInput {
    duplicate: bool,
    height: WindowTitleBarHeight,
    nested: bool,
    render: bool,
}

struct TitleBarComponent;

impl Component for TitleBarComponent {
    type Message = ();
    type Input = TitleBarInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        if !input.render {
            return TextBlock::new().text("content").into();
        }
        let title_bar = TitleBar::new()
            .title("Title")
            .preferred_height(input.height);
        let title_bar = title_bar.slots(std::iter::empty::<SlotView<TitleBarSlot>>());
        if input.duplicate {
            return StackPanel::new().children((title_bar, TitleBar::new()));
        }
        if input.nested {
            Border::new().content(title_bar)
        } else {
            title_bar
        }
    }
}

fn input() -> TitleBarInput {
    TitleBarInput {
        duplicate: false,
        height: WindowTitleBarHeight::Standard,
        nested: false,
        render: true,
    }
}

#[test]
fn direct_element_updates_reconcile_title_bar_declarations() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TitleBar::default().into()).unwrap();
    let window = pump.window.unwrap();
    let (title_bar, _) = pump.runtime().window_title_bar(window).unwrap();

    pump.update(
        TitleBar::new()
            .preferred_height(WindowTitleBarHeight::Tall)
            .into(),
    )
    .unwrap();
    assert_eq!(
        pump.runtime().window_title_bar(window),
        Some((title_bar, WindowTitleBarHeight::Tall))
    );

    pump.update(TextBlock::new().into()).unwrap();
    assert_eq!(pump.runtime().window_title_bar(window), None);
}

#[test]
fn title_bar_mount_update_clear_and_replacement_are_ordered() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<TitleBarComponent>(input()))
        .unwrap();
    let window = pump.window.unwrap();
    let (first, height) = pump.runtime().window_title_bar(window).unwrap();
    assert_eq!(height, WindowTitleBarHeight::Standard);
    let commands = &pump.runtime().commands()[0];
    let create = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Create {
                    node,
                    kind: MountedKind::TitleBar
                } if *node == first
            )
        })
        .unwrap();
    let set = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::SetWindowTitleBar { title_bar, .. } if *title_bar == first
            )
        })
        .unwrap();
    let activate = commands
        .iter()
        .position(|command| matches!(command, Command::ActivateWindow { .. }))
        .unwrap();
    assert!(create < set && set < activate);

    let mut tall = input();
    tall.height = WindowTitleBarHeight::Tall;
    pump.update_view(View::component::<TitleBarComponent>(tall))
        .unwrap();
    assert_eq!(
        pump.runtime().window_title_bar(window),
        Some((first, WindowTitleBarHeight::Tall))
    );

    let mut nested = tall;
    nested.nested = true;
    pump.update_view(View::component::<TitleBarComponent>(nested))
        .unwrap();
    let (second, height) = pump.runtime().window_title_bar(window).unwrap();
    assert_ne!(first, second);
    assert_eq!(height, WindowTitleBarHeight::Tall);
    let replacement = pump.runtime().commands().last().unwrap();
    let clear = replacement
        .iter()
        .position(|command| matches!(command, Command::ClearWindowTitleBar { .. }))
        .unwrap();
    let destroy = replacement
        .iter()
        .position(|command| matches!(command, Command::Destroy { node } if *node == first))
        .unwrap();
    let set = replacement
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::SetWindowTitleBar { title_bar, .. } if *title_bar == second
            )
        })
        .unwrap();
    assert!(clear < destroy && destroy < set);

    let mut removed = nested;
    removed.render = false;
    pump.update_view(View::component::<TitleBarComponent>(removed))
        .unwrap();
    let removal = pump.runtime().commands().last().unwrap();
    let clear = removal
        .iter()
        .position(|command| matches!(command, Command::ClearWindowTitleBar { .. }))
        .unwrap();
    let destroy = removal
        .iter()
        .position(|command| matches!(command, Command::Destroy { node } if *node == second))
        .unwrap();
    assert!(clear < destroy);
    assert_eq!(pump.runtime().window_title_bar(window), None);
    assert!(pump.tree.window_title_bar().is_none());

    pump.update_view(View::component::<TitleBarComponent>(input()))
        .unwrap();
    assert_eq!(
        pump.runtime()
            .window_title_bar(window)
            .map(|(_, height)| height),
        Some(WindowTitleBarHeight::Standard)
    );
}

#[test]
fn duplicate_title_bar_declarations_fail_before_native_apply() {
    let mut duplicate = input();
    duplicate.duplicate = true;
    let mut pump = Pump::new(RecordingRuntime::default());
    assert_eq!(
        pump.mount_view(View::component::<TitleBarComponent>(duplicate)),
        Err(PumpError::DuplicateWindowTitleBar)
    );
    assert!(pump.runtime().is_empty());
}

#[test]
fn failed_native_apply_does_not_publish_title_bar_height() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<TitleBarComponent>(input()))
        .unwrap();
    let window = pump.window.unwrap();
    let published = pump.tree.window_title_bar().unwrap();
    pump.runtime_mut().fail_at(0);

    let mut tall = input();
    tall.height = WindowTitleBarHeight::Tall;
    assert!(matches!(
        pump.update_view(View::component::<TitleBarComponent>(tall)),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert_eq!(
        pump.runtime().window_title_bar(window),
        Some((published.title_bar, WindowTitleBarHeight::Standard))
    );
    assert_eq!(pump.tree.window_title_bar().unwrap(), published);
}
