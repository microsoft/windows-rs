use std::cell::{Cell, RefCell};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use super::*;
use crate::element::tree::StructuralSlot;

#[test]
fn repeated_window_cycles_retire_header_and_pane_subtrees() {
    let open = Rc::new(RefCell::new(None::<State<bool>>));
    let open_for_render = Rc::clone(&open);
    let root = component(move |cx| {
        let current = cx.use_state(|| false);
        *open_for_render.borrow_mut() = Some(current.clone());
        let windows = if current.get().unwrap() {
            vec![
                Window::new(
                    "Leak regression",
                    StackPanel::new([
                        Expander::display(
                            StackPanel::new([
                                text_block("Header item 1"),
                                text_block("Header item 2"),
                                text_block("Header item 3"),
                            ])
                            .build(),
                            text_block("Expander content"),
                        )
                        .expanded(true)
                        .build(),
                        SplitView::display(
                            text_block("Split content"),
                            StackPanel::new([text_block("Pane item 1"), text_block("Pane item 2")])
                                .build(),
                        )
                        .is_pane_open(true)
                        .build(),
                    ])
                    .build(),
                    || {},
                )
                .build()
                .key(1),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let baseline = reactor.engine().node_count();
    assert_eq!(reactor.engine().runtime().native_node_count(), 0);

    for _ in 0..10 {
        assert!(open.borrow().as_ref().unwrap().try_set(true));
        reactor.pump();
        assert_eq!(reactor.engine().runtime().window_ids().len(), 1);
        assert!(reactor.engine().runtime().native_node_count() > 0);

        assert!(open.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        assert!(reactor.engine().runtime().window_ids().is_empty());
        assert_eq!(reactor.engine().runtime().native_node_count(), 0);
        assert_eq!(reactor.engine().node_count(), baseline);
    }
}

#[test]
fn handlers_and_effects_open_windows_through_application_state() {
    let root = component(|cx| {
        let handler_open = cx.use_state(|| false);
        let effect_open = cx.use_state(|| false);
        let open_from_effect = effect_open.clone();
        cx.use_effect((), move || {
            assert!(open_from_effect.try_set(true));
        });
        let open_from_handler = handler_open.clone();
        let mut windows = vec![
            Window::new(
                "Launcher",
                Button::new("Open handler window")
                    .on_click(move || {
                        assert!(open_from_handler.try_set(true));
                    })
                    .build(),
                || {},
            )
            .build()
            .key(1),
        ];
        if handler_open.get().unwrap() {
            windows.push(
                Window::new("Handler window", text_block("handler"), || {})
                    .build()
                    .key(2),
            );
        }
        if effect_open.get().unwrap() {
            windows.push(
                Window::new("Effect window", text_block("effect"), || {})
                    .build()
                    .key(3),
            );
        }
        Application::new(windows).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids().len(), 2);
    let button = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::Create {
                id,
                kind: NativeKind::Button,
            } => Some(*id),
            _ => None,
        })
        .unwrap();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target: button });
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids().len(), 3);
}

#[test]
fn windows_share_the_application_context_scope() {
    let context = Context::new(0usize);
    let first_context = context.clone();
    let second_context = context.clone();
    let observed = Rc::new(RefCell::new(Vec::new()));
    let first_observed = Rc::clone(&observed);
    let second_observed = Rc::clone(&observed);
    let root = provide_context(
        &context,
        42,
        Application::new([
            Window::new(
                "First",
                component(move |cx| {
                    first_observed
                        .borrow_mut()
                        .push(cx.use_context(&first_context));
                    text_block("first")
                }),
                || {},
            )
            .build()
            .key(1),
            Window::new(
                "Second",
                component(move |cx| {
                    second_observed
                        .borrow_mut()
                        .push(cx.use_context(&second_context));
                    text_block("second")
                }),
                || {},
            )
            .build()
            .key(2),
        ])
        .build(),
    );
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    assert_eq!(*observed.borrow(), [42, 42]);
    assert_eq!(reactor.engine().runtime().window_ids().len(), 2);
}

#[test]
fn application_resources_mount_update_and_clear_before_window_work() {
    let first = ApplicationResources::new([
        (
            "AccentBrush",
            ApplicationResource::from(Color::rgb(20, 40, 60)),
        ),
        ("Label", ApplicationResource::from("Primary")),
        ("Scale", ApplicationResource::from(1.5)),
        (
            "Padding",
            ApplicationResource::from(Thickness::uniform(8.0)),
        ),
    ]);
    let second = ApplicationResources::new([
        (
            "AccentBrush",
            ApplicationResource::from(Color::rgb(60, 40, 20)),
        ),
        ("Label", ApplicationResource::from("Secondary")),
    ]);
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let first_for_render = first.clone();
    let second_for_render = second.clone();
    let root = component(move |cx| {
        let current = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(current.clone());
        let resources = match current.get().unwrap() {
            0 => first_for_render.clone(),
            1 => second_for_render.clone(),
            _ => ApplicationResources::default(),
        };
        Application::new([Window::new("Main", text_block("main"), || {})
            .build()
            .key(1)])
        .resources(resources)
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    let application = reactor.engine().parent(window).unwrap();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .application_resources(application),
        Some(&first)
    );
    assert!(matches!(
        reactor.engine().runtime().batches().first().unwrap().first(),
        Some(Command::UpdateApplication {
            id,
            update: ApplicationUpdate::Resources(resources)
        }) if *id == application && **resources == first
    ));

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .application_resources(application),
        Some(&second)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .application_resources(application),
        Some(&ApplicationResources::default())
    );
}

#[test]
fn application_resources_reject_invalid_entries() {
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            ApplicationResources::new([
                ("Duplicate", ApplicationResource::from("first")),
                ("Duplicate", ApplicationResource::from("second")),
            ])
        }))
        .is_err()
    );
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            ApplicationResources::new([("", ApplicationResource::from("value"))])
        }))
        .is_err()
    );
    assert!(catch_unwind(AssertUnwindSafe(|| ApplicationResource::from(f64::NAN))).is_err());
}

#[test]
fn application_mounts_keyed_windows_in_one_runtime() {
    let root = Application::new([
        Window::new("Main", text_block("main"), || {})
            .build()
            .key(1),
        Window::new("Inspector", text_block("inspector"), || {})
            .build()
            .key(2),
    ])
    .build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    let runtime = reactor.engine().runtime();
    let windows = runtime.window_ids();
    assert_eq!(windows.len(), 2);
    assert_eq!(runtime.window_title(windows[0]), Some("Main"));
    assert_eq!(runtime.window_title(windows[1]), Some("Inspector"));
    assert_eq!(
        reactor.engine().node_kind(windows[0]),
        Some(&NodeKind::Window)
    );
    assert_eq!(
        reactor.engine().node_kind(windows[1]),
        Some(&NodeKind::Window)
    );
    let application = reactor.engine().parent(windows[0]).unwrap();
    assert_eq!(reactor.engine().parent(windows[1]), Some(application));
    assert_eq!(
        reactor.engine().node_kind(application),
        Some(&NodeKind::Application)
    );
    let main = runtime.window_content(windows[0]).unwrap();
    let inspector = runtime.window_content(windows[1]).unwrap();
    assert_eq!(reactor.engine().parent(main), Some(windows[0]));
    assert_eq!(reactor.engine().parent(inspector), Some(windows[1]));
    assert_ne!(main, inspector);
    assert!(runtime.contains(main));
    assert!(runtime.contains(inspector));
}

#[test]
fn window_title_and_content_update_without_replacing_window_identity() {
    let title = Rc::new(RefCell::new(None::<State<String>>));
    let alternate = Rc::new(RefCell::new(None::<State<bool>>));
    let title_for_render = Rc::clone(&title);
    let alternate_for_render = Rc::clone(&alternate);
    let root = component(move |cx| {
        let current_title = cx.use_state(|| "Main".to_string());
        let current_alternate = cx.use_state(|| false);
        *title_for_render.borrow_mut() = Some(current_title.clone());
        *alternate_for_render.borrow_mut() = Some(current_alternate.clone());
        let content = if current_alternate.get().unwrap() {
            Button::new("replacement").on_click(|| {}).build()
        } else {
            TextBlock::new("initial").build()
        };
        Window::new(current_title.get().unwrap(), content, || {})
            .build()
            .key(1)
    });
    let root = Application::new([root]).build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    let original_content = reactor.engine().runtime().window_content(window).unwrap();
    let original_body = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::Create {
                id,
                kind: NativeKind::TextBlock,
            } => Some(*id),
            _ => None,
        })
        .unwrap();
    assert!(
        title
            .borrow()
            .as_ref()
            .unwrap()
            .try_set("Renamed".to_string())
    );
    assert!(alternate.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    let runtime = reactor.engine().runtime();
    assert_eq!(runtime.window_ids(), [window]);
    assert_eq!(runtime.window_title(window), Some("Renamed"));
    let new_body = runtime
        .batches()
        .last()
        .unwrap()
        .iter()
        .find_map(|command| match command {
            Command::Create {
                id,
                kind: NativeKind::Button,
            } => Some(*id),
            _ => None,
        })
        .unwrap();
    let replacement = runtime.window_content(window).unwrap();
    assert_eq!(replacement, original_content);
    assert!(!runtime.contains(original_body));
    assert!(runtime.contains(new_body));
    assert!(runtime.contains(replacement));
    assert!(
        runtime
            .batches()
            .last()
            .unwrap()
            .iter()
            .all(|command| !matches!(command, Command::SetWindowContent { .. }))
    );
}

#[test]
fn window_client_size_and_presenter_update_without_replacement() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let current = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(current.clone());
        let window = Window::new("Main", text_block("main"), || {});
        let window = match current.get().unwrap() {
            0 => window
                .client_size(640.0, 480.0)
                .presenter(WindowPresenter::FullScreen),
            1 => window
                .client_size(800.0, 600.0)
                .presenter(WindowPresenter::CompactOverlay),
            _ => window,
        };
        Application::new([window.build().key(1)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    assert_eq!(
        reactor.engine().runtime().window_size(window),
        Some(WindowSize {
            width: 640.0,
            height: 480.0,
        })
    );
    assert_eq!(
        reactor.engine().runtime().window_presenter(window),
        Some(WindowPresenter::FullScreen)
    );
    let first_batch = reactor.engine().runtime().batches().first().unwrap();
    let content = first_batch
        .iter()
        .position(|command| matches!(command, Command::SetWindowContent { .. }))
        .unwrap();
    let presenter = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::Presenter(_),
                    ..
                }
            )
        })
        .unwrap();
    let size = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::ClientSize(_),
                    ..
                }
            )
        })
        .unwrap();
    assert!(presenter < size && size < content);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids(), [window]);
    assert_eq!(
        reactor.engine().runtime().window_size(window),
        Some(WindowSize {
            width: 800.0,
            height: 600.0,
        })
    );
    assert_eq!(
        reactor.engine().runtime().window_presenter(window),
        Some(WindowPresenter::CompactOverlay)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().window_size(window),
        Some(WindowSize {
            width: 800.0,
            height: 600.0,
        })
    );
    assert_eq!(
        reactor.engine().runtime().window_presenter(window),
        Some(WindowPresenter::Default)
    );
}

#[test]
fn window_rejects_invalid_client_size() {
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {}).client_size(f64::NAN, 480.0)
        }))
        .is_err()
    );
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {}).client_size(640.0, 0.0)
        }))
        .is_err()
    );
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {}).client_size(i32::MAX as f64 + 1.0, 480.0)
        }))
        .is_err()
    );
}

#[test]
fn window_backdrop_updates_and_clears_without_replacement() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let current = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(current.clone());
        let window = Window::new("Main", text_block("main"), || {});
        let window = match current.get().unwrap() {
            0 => window.backdrop(WindowBackdrop::Mica),
            1 => window.backdrop(WindowBackdrop::Acrylic),
            _ => window,
        };
        Application::new([window.build().key(1)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    assert_eq!(
        reactor.engine().runtime().window_backdrop(window),
        Some(WindowBackdrop::Mica)
    );
    let first_batch = reactor.engine().runtime().batches().first().unwrap();
    let backdrop = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::Backdrop(_),
                    ..
                }
            )
        })
        .unwrap();
    let content = first_batch
        .iter()
        .position(|command| matches!(command, Command::SetWindowContent { .. }))
        .unwrap();
    assert!(backdrop < content);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids(), [window]);
    assert_eq!(
        reactor.engine().runtime().window_backdrop(window),
        Some(WindowBackdrop::Acrylic)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids(), [window]);
    assert_eq!(reactor.engine().runtime().window_backdrop(window), None);
    assert!(matches!(
        reactor.engine().runtime().batches().last().unwrap().as_slice(),
        [Command::UpdateWindow {
            id,
            update: WindowUpdate::Backdrop(None)
        }] if *id == window
    ));
}

#[test]
fn window_icon_updates_without_replacement_and_omission_keeps_the_last_request() {
    let first = WindowIcon::file(r"C:\icons\first.ico");
    let second = WindowIcon::file(r"C:\icons\second.ICO");
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let first_for_render = first.clone();
    let second_for_render = second.clone();
    let root = component(move |cx| {
        let current = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(current.clone());
        let window = Window::new("Main", text_block("main"), || {});
        let window = match current.get().unwrap() {
            0 => window.icon(first_for_render.clone()),
            1 => window.icon(second_for_render.clone()),
            _ => window,
        };
        Application::new([window.build().key(1)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    assert_eq!(reactor.engine().runtime().window_icon(window), Some(&first));
    let first_batch = reactor.engine().runtime().batches().first().unwrap();
    let icon = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::Icon(_),
                    ..
                }
            )
        })
        .unwrap();
    let content = first_batch
        .iter()
        .position(|command| matches!(command, Command::SetWindowContent { .. }))
        .unwrap();
    assert!(icon < content);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids(), [window]);
    assert_eq!(
        reactor.engine().runtime().window_icon(window),
        Some(&second)
    );

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids(), [window]);
    assert_eq!(
        reactor.engine().runtime().window_icon(window),
        Some(&second)
    );
    assert_eq!(reactor.engine().runtime().batches().len(), batch_count);
}

#[test]
fn window_icon_requires_an_absolute_ico_path() {
    assert!(catch_unwind(AssertUnwindSafe(|| WindowIcon::file("relative.ico"))).is_err());
    assert!(
        catch_unwind(AssertUnwindSafe(|| WindowIcon::file(
            r"C:\icons\window.png"
        )))
        .is_err()
    );
}

#[test]
fn window_theme_updates_and_resets_without_replacement() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let current = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(current.clone());
        let window = Window::new("Main", text_block("main"), || {});
        let window = match current.get().unwrap() {
            0 => window.theme(WindowTheme::Dark),
            1 => window.theme(WindowTheme::Light),
            _ => window,
        };
        Application::new([window.build().key(1)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    assert_eq!(
        reactor.engine().runtime().window_theme(window),
        Some(WindowTheme::Dark)
    );
    let first_batch = reactor.engine().runtime().batches().first().unwrap();
    let theme = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::Theme(_),
                    ..
                }
            )
        })
        .unwrap();
    let content = first_batch
        .iter()
        .position(|command| matches!(command, Command::SetWindowContent { .. }))
        .unwrap();
    assert!(theme < content);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids(), [window]);
    assert_eq!(
        reactor.engine().runtime().window_theme(window),
        Some(WindowTheme::Light)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().window_theme(window),
        Some(WindowTheme::System)
    );
    assert!(matches!(
        reactor.engine().runtime().batches().last().unwrap().as_slice(),
        [Command::UpdateWindow {
            id,
            update: WindowUpdate::Theme(WindowTheme::System)
        }] if *id == window
    ));
}

#[test]
fn window_title_bar_updates_and_resets_without_replacement() {
    let first = SystemTitleBar {
        extend_content: true,
        height: TitleBarHeight::Tall,
        icon: SystemTitleBarIconPolicy::HideIconAndSystemMenu,
        colors: SystemTitleBarColors {
            foreground: Some(Color::rgb(255, 255, 255)),
            background: Some(Color::rgb(20, 40, 60)),
            button_hover_background: Some(Color::rgb(40, 80, 120)),
            ..Default::default()
        },
        ..Default::default()
    };
    let second = SystemTitleBar {
        extend_content: true,
        buttons: SystemTitleBarButtonPolicy::Hidden,
        colors: SystemTitleBarColors {
            background: Some(Color::rgb(10, 20, 30)),
            ..Default::default()
        },
        ..Default::default()
    };
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let current = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(current.clone());
        let window = Window::new("Main", text_block("main"), || {});
        let window = match current.get().unwrap() {
            0 => window.title_bar(first),
            1 => window.title_bar(second),
            _ => window.presenter(WindowPresenter::FullScreen),
        };
        Application::new([window.build().key(1)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    assert_eq!(
        reactor.engine().runtime().window_title_bar(window),
        Some(first)
    );
    let first_batch = reactor.engine().runtime().batches().first().unwrap();
    let title_bar = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::TitleBar(_),
                    ..
                }
            )
        })
        .unwrap();
    let content = first_batch
        .iter()
        .position(|command| matches!(command, Command::SetWindowContent { .. }))
        .unwrap();
    assert!(title_bar < content);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids(), [window]);
    assert_eq!(
        reactor.engine().runtime().window_title_bar(window),
        Some(second)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().window_title_bar(window),
        Some(SystemTitleBar::default())
    );
    assert!(matches!(
        reactor.engine().runtime().batches().last().unwrap().as_slice(),
        [
            Command::UpdateWindow {
                id: title_bar_id,
                update: WindowUpdate::TitleBar(title_bar)
            },
            Command::UpdateWindow {
                id: presenter_id,
                update: WindowUpdate::Presenter(WindowPresenter::FullScreen)
            }
        ] if *title_bar_id == window
            && *presenter_id == window
            && **title_bar == SystemTitleBar::default()
    ));
}

#[test]
fn window_rejects_invalid_title_bar_options() {
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {}).title_bar(SystemTitleBar {
                height: TitleBarHeight::Tall,
                ..Default::default()
            })
        }))
        .is_err()
    );
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {}).title_bar(SystemTitleBar {
                buttons: SystemTitleBarButtonPolicy::Hidden,
                ..Default::default()
            })
        }))
        .is_err()
    );
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {}).title_bar(SystemTitleBar {
                extend_content: true,
                height: TitleBarHeight::Tall,
                buttons: SystemTitleBarButtonPolicy::Hidden,
                ..Default::default()
            })
        }))
        .is_err()
    );
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {})
                .title_bar(SystemTitleBar {
                    colors: SystemTitleBarColors {
                        background: Some(Color::rgb(10, 20, 30)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .presenter(WindowPresenter::FullScreen)
        }))
        .is_err()
    );
}

#[test]
fn custom_title_bar_mounts_updates_switches_and_unmounts_in_order() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let back_count = Rc::new(Cell::new(0));
    let pane_count = Rc::new(Cell::new(0));
    let body_value = Rc::new(RefCell::new(None::<State<i32>>));
    let back_for_render = Rc::clone(&back_count);
    let pane_for_render = Rc::clone(&pane_count);
    let body_for_render = Rc::clone(&body_value);
    let root = component(move |cx| {
        let current = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(current.clone());
        let phase = current.get().unwrap();
        if phase == 5 {
            return Application::new([]).build();
        }
        let publish_body = Rc::clone(&body_for_render);
        let body = component(move |cx| {
            let value = cx.use_state(|| 0);
            publish_body.borrow_mut().replace(value.clone());
            text_block(format!("body = {}", value.value()))
        });
        let window = Window::new("Main", body, || {});
        let window = match phase {
            0 => {
                let back = Rc::clone(&back_for_render);
                let pane = Rc::clone(&pane_for_render);
                window.title_bar(
                    TitleBar::custom("First")
                        .subtitle(Some("Subtitle".to_string()))
                        .content(Some(text_block("Center").key(10)))
                        .right_header(Some(text_block("Right").key(20)))
                        .back_button_visible(true)
                        .pane_toggle_button_visible(true)
                        .height(TitleBarHeight::Tall)
                        .on_back_requested(move || back.set(back.get() + 1))
                        .on_pane_requested(move || pane.set(pane.get() + 1)),
                )
            }
            1 => {
                let back = Rc::clone(&back_for_render);
                let pane = Rc::clone(&pane_for_render);
                window.title_bar(
                    TitleBar::custom("Second")
                        .right_header(Some(text_block("Updated").key(20)))
                        .back_button_visible(true)
                        .pane_toggle_button_visible(true)
                        .on_back_requested(move || back.set(back.get() + 10))
                        .on_pane_requested(move || pane.set(pane.get() + 10)),
                )
            }
            2 => {
                let back = Rc::clone(&back_for_render);
                let pane = Rc::clone(&pane_for_render);
                window.title_bar(
                    TitleBar::custom("Content only")
                        .content(Some(text_block("Replacement center").key(10)))
                        .back_button_visible(true)
                        .pane_toggle_button_visible(true)
                        .on_back_requested(move || back.set(back.get() + 100))
                        .on_pane_requested(move || pane.set(pane.get() + 100)),
                )
            }
            3 => window,
            _ => window.title_bar(TitleBar::custom("Mounted again")),
        };
        Application::new([window.build().key(1)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    let chrome = reactor.engine().runtime().window_title_bar(window).unwrap();
    assert_eq!(chrome.colors.background, Some(Color::argb(0, 0, 0, 0)));
    assert_eq!(
        chrome.colors.button_background,
        Some(Color::argb(0, 0, 0, 0))
    );
    assert_eq!(
        chrome.colors.inactive_background,
        Some(Color::argb(0, 0, 0, 0))
    );
    assert_eq!(
        chrome.colors.button_inactive_background,
        Some(Color::argb(0, 0, 0, 0))
    );
    let title_bar = reactor
        .engine()
        .runtime()
        .window_custom_title_bar(window)
        .unwrap();
    let [content_slot, right_header_slot] = *reactor
        .engine()
        .arena
        .get(title_bar)
        .unwrap()
        .children
        .as_slice()
    else {
        unreachable!()
    };
    assert_eq!(
        reactor.engine().node_kind(content_slot),
        Some(&NodeKind::StructuralSlot(StructuralSlot::Content))
    );
    assert_eq!(
        reactor.engine().node_kind(right_header_slot),
        Some(&NodeKind::StructuralSlot(StructuralSlot::Pane))
    );
    assert_eq!(
        reactor.engine().runtime().kind(title_bar),
        Some(NativeKind::TitleBar)
    );
    let slots = reactor.engine().runtime().children(title_bar);
    assert_eq!(slots.len(), 2);
    let initial_content = slots[0];
    let initial_right_header = slots[1];
    assert_eq!(
        reactor.engine().runtime().attachment(initial_content),
        Some(Attachment::Content)
    );
    assert_eq!(
        reactor.engine().runtime().attachment(initial_right_header),
        Some(Attachment::Pane)
    );
    let first_batch = reactor.engine().runtime().batches().first().unwrap();
    let bind = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::BindTitleBar(id),
                    ..
                } if *id == title_bar
            )
        })
        .unwrap();
    let content = first_batch
        .iter()
        .position(|command| matches!(command, Command::SetWindowContent { .. }))
        .unwrap();
    assert!(bind < content);
    assert!(body_value.borrow().as_ref().unwrap().try_set(42));
    reactor.pump();
    assert_eq!(body_value.borrow().as_ref().unwrap().value(), 42);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TitleBarBackRequested { target: title_bar });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TitleBarPaneRequested { target: title_bar });
    reactor.pump();
    assert_eq!(back_count.get(), 1);
    assert_eq!(pane_count.get(), 1);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().window_custom_title_bar(window),
        Some(title_bar)
    );
    let slots = reactor.engine().runtime().children(title_bar);
    assert_eq!(slots, [initial_right_header]);
    assert!(!reactor.engine().runtime().contains(initial_content));
    assert_eq!(
        reactor.engine().runtime().attachment(slots[0]),
        Some(Attachment::Pane)
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TitleBarBackRequested { target: title_bar });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TitleBarPaneRequested { target: title_bar });
    reactor.pump();
    assert_eq!(back_count.get(), 11);
    assert_eq!(pane_count.get(), 11);
    assert_eq!(body_value.borrow().as_ref().unwrap().value(), 42);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().window_custom_title_bar(window),
        Some(title_bar)
    );
    let slots = reactor.engine().runtime().children(title_bar);
    assert_eq!(slots.len(), 1);
    assert_eq!(
        reactor.engine().runtime().attachment(slots[0]),
        Some(Attachment::Content)
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TitleBarBackRequested { target: title_bar });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TitleBarPaneRequested { target: title_bar });
    reactor.pump();
    assert_eq!(back_count.get(), 111);
    assert_eq!(pane_count.get(), 111);
    assert_eq!(body_value.borrow().as_ref().unwrap().value(), 42);

    assert!(phase.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().window_custom_title_bar(window),
        None
    );
    assert!(!reactor.engine().runtime().contains(title_bar));
    assert_eq!(body_value.borrow().as_ref().unwrap().value(), 42);
    let switch_batch = reactor.engine().runtime().batches().last().unwrap();
    let unbind = switch_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::UnbindTitleBar,
                    ..
                }
            )
        })
        .unwrap();
    let destroy = switch_batch
        .iter()
        .position(|command| matches!(command, Command::Destroy { id } if *id == title_bar))
        .unwrap();
    assert!(unbind < destroy);

    assert!(phase.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    let replacement = reactor
        .engine()
        .runtime()
        .window_custom_title_bar(window)
        .unwrap();
    assert_ne!(replacement, title_bar);
    assert_eq!(body_value.borrow().as_ref().unwrap().value(), 42);
    let switch_batch = reactor.engine().runtime().batches().last().unwrap();
    let bind = switch_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::BindTitleBar(id),
                    ..
                } if *id == replacement
            )
        })
        .unwrap();
    assert!(
        switch_batch[bind + 1..]
            .iter()
            .all(|command| !matches!(command, Command::SetWindowContent { .. }))
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(5));
    reactor.pump();
    assert!(reactor.engine().runtime().window_ids().is_empty());
    let removal_batch = reactor.engine().runtime().batches().last().unwrap();
    let unbind = removal_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::UnbindTitleBar,
                    ..
                }
            )
        })
        .unwrap();
    let destroy = removal_batch
        .iter()
        .position(|command| matches!(command, Command::Destroy { id } if *id == replacement))
        .unwrap();
    let close = removal_batch
        .iter()
        .position(|command| matches!(command, Command::CloseWindow { id } if *id == window))
        .unwrap();
    assert!(unbind < destroy);
    assert!(destroy < close);
}

#[test]
fn window_overlapped_policy_updates_and_clears_before_presenter_change() {
    let first = WindowOverlappedPolicy {
        resizable: false,
        minimizable: true,
        maximizable: false,
    };
    let second = WindowOverlappedPolicy {
        resizable: true,
        minimizable: false,
        maximizable: true,
    };
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let current = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(current.clone());
        let window = Window::new("Main", text_block("main"), || {});
        let window = match current.get().unwrap() {
            0 => window.overlapped(first),
            1 => window.overlapped(second),
            _ => window.presenter(WindowPresenter::CompactOverlay),
        };
        Application::new([window.build().key(1)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    assert_eq!(
        reactor.engine().runtime().window_overlapped(window),
        Some(first)
    );
    let first_batch = reactor.engine().runtime().batches().first().unwrap();
    let policy = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::Overlapped(_),
                    ..
                }
            )
        })
        .unwrap();
    let content = first_batch
        .iter()
        .position(|command| matches!(command, Command::SetWindowContent { .. }))
        .unwrap();
    assert!(policy < content);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().window_overlapped(window),
        Some(second)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().window_overlapped(window),
        Some(WindowOverlappedPolicy::default())
    );
    assert!(matches!(
        reactor.engine().runtime().batches().last().unwrap().as_slice(),
        [
            Command::UpdateWindow {
                id: policy_id,
                update: WindowUpdate::Overlapped(policy)
            },
            Command::UpdateWindow {
                id: presenter_id,
                update: WindowUpdate::Presenter(WindowPresenter::CompactOverlay)
            }
        ] if *policy_id == window
            && *presenter_id == window
            && *policy == WindowOverlappedPolicy::default()
    ));
}

#[test]
fn window_rejects_overlapped_policy_with_non_default_presenter() {
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {})
                .overlapped(WindowOverlappedPolicy {
                    resizable: false,
                    ..Default::default()
                })
                .presenter(WindowPresenter::FullScreen)
        }))
        .is_err()
    );
}

#[test]
fn window_constraints_update_and_clear_without_replacement() {
    let constrained = Rc::new(RefCell::new(None::<State<bool>>));
    let constrained_for_render = Rc::clone(&constrained);
    let constraints = WindowConstraints {
        min_width: Some(320.0),
        min_height: Some(240.0),
        max_width: Some(1280.0),
        max_height: Some(960.0),
    };
    let root = component(move |cx| {
        let current = cx.use_state(|| true);
        *constrained_for_render.borrow_mut() = Some(current.clone());
        let window = Window::new("Main", text_block("main"), || {}).client_size(640.0, 480.0);
        Application::new([if current.get().unwrap() {
            window.client_constraints(constraints).build().key(1)
        } else {
            window.presenter(WindowPresenter::FullScreen).build().key(1)
        }])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = reactor.engine().runtime().window_ids()[0];
    assert_eq!(
        reactor.engine().runtime().window_constraints(window),
        Some(constraints)
    );
    let first_batch = reactor.engine().runtime().batches().first().unwrap();
    let constraints_position = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::Constraints(_),
                    ..
                }
            )
        })
        .unwrap();
    let size_position = first_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::ClientSize(_),
                    ..
                }
            )
        })
        .unwrap();
    assert!(constraints_position < size_position);

    assert!(constrained.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_ids(), [window]);
    assert_eq!(
        reactor.engine().runtime().window_constraints(window),
        Some(WindowConstraints::default())
    );
    assert_eq!(
        reactor.engine().runtime().window_presenter(window),
        Some(WindowPresenter::FullScreen)
    );
    let update_batch = reactor.engine().runtime().batches().last().unwrap();
    let constraints_position = update_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::Constraints(_),
                    ..
                }
            )
        })
        .unwrap();
    let presenter_position = update_batch
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UpdateWindow {
                    update: WindowUpdate::Presenter(_),
                    ..
                }
            )
        })
        .unwrap();
    assert!(constraints_position < presenter_position);
}

#[test]
fn window_rejects_invalid_constraints_and_presenter_combinations() {
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {}).client_constraints(WindowConstraints {
                min_width: Some(640.0),
                max_width: Some(320.0),
                ..Default::default()
            })
        }))
        .is_err()
    );
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            Window::new("Main", text_block("main"), || {})
                .client_constraints(WindowConstraints {
                    min_width: Some(320.0),
                    ..Default::default()
                })
                .presenter(WindowPresenter::FullScreen)
        }))
        .is_err()
    );
}

#[test]
fn close_request_uses_the_latest_callback_and_declarative_removal() {
    let close_count = Rc::new(Cell::new(0));
    let open = Rc::new(RefCell::new(None::<State<bool>>));
    let increment = Rc::new(RefCell::new(None::<State<u32>>));
    let close_count_for_render = Rc::clone(&close_count);
    let open_for_render = Rc::clone(&open);
    let increment_for_render = Rc::clone(&increment);
    let root = component(move |cx| {
        let current_open = cx.use_state(|| true);
        let current_increment = cx.use_state(|| 1);
        *open_for_render.borrow_mut() = Some(current_open.clone());
        *increment_for_render.borrow_mut() = Some(current_increment.clone());
        let windows = if current_open.get().unwrap() {
            let close_count = Rc::clone(&close_count_for_render);
            let close = current_open;
            let increment = current_increment.get().unwrap();
            vec![
                Window::new("Main", text_block("main"), move || {
                    close_count.set(close_count.get() + increment);
                    assert!(close.try_set(false));
                })
                .build()
                .key(1),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let window = reactor.engine().runtime().window_ids()[0];
    assert!(increment.borrow().as_ref().unwrap().try_set(10));
    reactor.pump();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::WindowCloseRequested { target: window });
    reactor.pump();

    assert_eq!(close_count.get(), 10);
    assert_eq!(open.borrow().as_ref().unwrap().get(), Some(false));
    assert!(reactor.engine().runtime().window_ids().is_empty());
}

#[test]
fn window_size_change_uses_the_latest_callback() {
    let observed = Rc::new(Cell::new(WindowSize {
        width: 0.0,
        height: 0.0,
    }));
    let multiplier = Rc::new(RefCell::new(None::<State<f64>>));
    let observed_for_render = Rc::clone(&observed);
    let multiplier_for_render = Rc::clone(&multiplier);
    let root = component(move |cx| {
        let current_multiplier = cx.use_state(|| 1.0);
        *multiplier_for_render.borrow_mut() = Some(current_multiplier.clone());
        let multiplier = current_multiplier.get().unwrap();
        let observed = Rc::clone(&observed_for_render);
        Application::new([Window::new("Main", text_block("main"), || {})
            .on_size_changed(move |size| {
                observed.set(WindowSize {
                    width: size.width * multiplier,
                    height: size.height * multiplier,
                });
            })
            .build()])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let window = reactor.engine().runtime().window_ids()[0];
    assert!(multiplier.borrow().as_ref().unwrap().try_set(2.0));
    reactor.pump();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::WindowSizeChanged {
            target: window,
            size: WindowSize {
                width: 400.0,
                height: 300.0,
            },
        });
    reactor.pump();

    assert_eq!(
        observed.get(),
        WindowSize {
            width: 800.0,
            height: 600.0
        }
    );
}

#[test]
fn color_scheme_change_uses_the_latest_callback() {
    let observed = Rc::new(Cell::new((ColorScheme::Light, 0)));
    let revision = Rc::new(RefCell::new(None::<State<u32>>));
    let observed_for_render = Rc::clone(&observed);
    let revision_for_render = Rc::clone(&revision);
    let root = component(move |cx| {
        let current_revision = cx.use_state(|| 0);
        *revision_for_render.borrow_mut() = Some(current_revision.clone());
        let revision = current_revision.get().unwrap();
        let observed = Rc::clone(&observed_for_render);
        Application::new([Window::new("Main", text_block("main"), || {})
            .on_color_scheme_changed(move |scheme| observed.set((scheme, revision)))
            .build()])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let window = reactor.engine().runtime().window_ids()[0];
    assert!(revision.borrow().as_ref().unwrap().try_set(7));
    reactor.pump();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::WindowColorSchemeChanged {
            target: window,
            scheme: ColorScheme::Dark,
        });
    reactor.pump();

    assert_eq!(observed.get(), (ColorScheme::Dark, 7));
}

#[test]
fn keyed_window_reorder_preserves_window_identity() {
    let reversed = Rc::new(RefCell::new(None::<State<bool>>));
    let reversed_for_render = Rc::clone(&reversed);
    let root = component(move |cx| {
        let current = cx.use_state(|| false);
        *reversed_for_render.borrow_mut() = Some(current.clone());
        let main = Window::new("Main", text_block("main"), || {})
            .build()
            .key(1);
        let inspector = Window::new("Inspector", text_block("inspector"), || {})
            .build()
            .key(2);
        Application::new(if current.get().unwrap() {
            vec![inspector, main]
        } else {
            vec![main, inspector]
        })
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let before = reactor
        .engine()
        .runtime()
        .window_ids()
        .into_iter()
        .map(|id| {
            (
                reactor
                    .engine()
                    .runtime()
                    .window_title(id)
                    .unwrap()
                    .to_string(),
                id,
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();

    assert!(reversed.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    let after = reactor
        .engine()
        .runtime()
        .window_ids()
        .into_iter()
        .map(|id| {
            (
                reactor
                    .engine()
                    .runtime()
                    .window_title(id)
                    .unwrap()
                    .to_string(),
                id,
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(after, before);
}

#[test]
fn application_rejects_native_controls_outside_windows() {
    let root = Application::new([text_block("orphan")]).build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    assert!(catch_unwind(AssertUnwindSafe(|| reactor.pump())).is_err());
}

#[test]
fn application_rejects_unkeyed_multiple_windows() {
    let root = Application::new([
        Window::new("One", text_block("one"), || {}).build(),
        Window::new("Two", text_block("two"), || {}).build(),
    ])
    .build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    assert!(catch_unwind(AssertUnwindSafe(|| reactor.pump())).is_err());
}

#[test]
fn application_rejects_duplicate_window_keys_through_transparent_nodes() {
    let root = Application::new([
        fragment([Window::new("One", text_block("one"), || {}).build().key(1)]),
        fragment([Window::new("Two", text_block("two"), || {}).build().key(1)]),
    ])
    .build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    assert!(catch_unwind(AssertUnwindSafe(|| reactor.pump())).is_err());
}

#[test]
fn window_rejects_content_with_multiple_native_roots() {
    let content: Element = fragment([text_block("one"), text_block("two")]);
    let root = Application::new([Window::new("Main", content, || {}).build()]).build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    assert!(catch_unwind(AssertUnwindSafe(|| reactor.pump())).is_err());
}

#[test]
fn application_revalidates_after_structural_changes() {
    let invalid = Rc::new(RefCell::new(None::<State<bool>>));
    let invalid_for_render = Rc::clone(&invalid);
    let root = component(move |cx| {
        let current = cx.use_state(|| false);
        *invalid_for_render.borrow_mut() = Some(current.clone());
        if current.get().unwrap() {
            Application::new([text_block("orphan")]).build()
        } else {
            Application::new([Window::new("Main", text_block("main"), || {}).build()]).build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(invalid.borrow().as_ref().unwrap().try_set(true));
    assert!(catch_unwind(AssertUnwindSafe(|| reactor.pump())).is_err());
}

#[test]
fn stale_close_request_is_ignored_after_window_removal() {
    let open = Rc::new(RefCell::new(None::<State<bool>>));
    let open_for_render = Rc::clone(&open);
    let root = component(move |cx| {
        let current = cx.use_state(|| true);
        *open_for_render.borrow_mut() = Some(current.clone());
        Application::new(if current.get().unwrap() {
            vec![
                Window::new("Main", text_block("main"), || {})
                    .build()
                    .key(1),
            ]
        } else {
            Vec::new()
        })
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stale = reactor.engine().runtime().window_ids()[0];
    assert!(open.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    let batches = reactor.engine().runtime().batches().len();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::WindowCloseRequested { target: stale });
    reactor.pump();

    assert!(reactor.engine().runtime().window_ids().is_empty());
    assert_eq!(reactor.engine().runtime().batches().len(), batches);
}

#[test]
fn owned_windows_close_before_content_and_parent() {
    let open = Rc::new(RefCell::new(None::<State<bool>>));
    let open_for_render = Rc::clone(&open);
    let root = component(move |cx| {
        let current = cx.use_state(|| true);
        *open_for_render.borrow_mut() = Some(current.clone());
        Application::new(if current.get().unwrap() {
            let close = current;
            vec![
                Window::new("Parent", text_block("parent"), move || {
                    assert!(close.try_set(false));
                })
                .owned_windows([Window::new("Owned", text_block("owned"), || {})
                    .build()
                    .key(2)])
                .build()
                .key(1),
            ]
        } else {
            Vec::new()
        })
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let windows = reactor.engine().runtime().window_ids();
    let parent = windows
        .iter()
        .copied()
        .find(|id| reactor.engine().runtime().window_title(*id) == Some("Parent"))
        .unwrap();
    let owned = windows
        .iter()
        .copied()
        .find(|id| reactor.engine().runtime().window_title(*id) == Some("Owned"))
        .unwrap();
    assert_eq!(reactor.engine().runtime().window_owner(owned), Some(parent));

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::WindowCloseRequested { target: parent });
    reactor.pump();

    assert!(reactor.engine().runtime().window_ids().is_empty());
    let closes = reactor
        .engine()
        .runtime()
        .batches()
        .last()
        .unwrap()
        .iter()
        .filter_map(|command| match command {
            Command::CloseWindow { id } => Some(*id),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(closes, [owned, parent]);
}

#[test]
fn owned_window_component_can_open_after_parent_mount() {
    let open = Rc::new(RefCell::new(None::<State<bool>>));
    let open_for_render = Rc::clone(&open);
    let owned = component(move |cx| {
        let current = cx.use_state(|| false);
        *open_for_render.borrow_mut() = Some(current.clone());
        if current.get().unwrap() {
            Window::new("Owned", text_block("owned"), || {})
                .build()
                .key(2)
        } else {
            fragment([])
        }
    });
    let root = Application::new([Window::new("Parent", text_block("parent"), || {})
        .owned_windows([owned])
        .build()
        .key(1)])
    .build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let parent = reactor.engine().runtime().window_ids()[0];

    assert!(open.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    let owned = reactor
        .engine()
        .runtime()
        .window_ids()
        .into_iter()
        .find(|id| *id != parent)
        .unwrap();
    assert_eq!(reactor.engine().runtime().window_owner(owned), Some(parent));
}

#[test]
fn owned_window_keys_are_scoped_to_their_owner_slot() {
    let root = Application::new([Window::new("Parent", text_block("parent"), || {})
        .owned_windows([Window::new("Owned", text_block("owned"), || {})
            .build()
            .key(1)])
        .build()
        .key(1)])
    .build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    assert_eq!(reactor.engine().runtime().window_ids().len(), 2);
}

#[test]
fn window_reference_activates_only_its_live_generation() {
    let reference = WindowRef::new();
    let render_reference = reference.clone();
    let open = Rc::new(RefCell::new(None::<State<bool>>));
    let open_for_render = Rc::clone(&open);
    let root = component(move |cx| {
        let current = cx.use_state(|| true);
        *open_for_render.borrow_mut() = Some(current.clone());
        Application::new(if current.get().unwrap() {
            vec![
                Window::new("Main", text_block("main"), || {})
                    .reference(&render_reference)
                    .build()
                    .key(1),
            ]
        } else {
            Vec::new()
        })
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    assert!(!reference.activate());
    reactor.pump();

    let first = reference.node().unwrap();
    assert!(reference.activate());
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_activations(), [first]);

    assert!(open.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert!(!reference.is_mounted());
    assert!(!reference.activate());

    assert!(open.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    let second = reference.node().unwrap();
    assert_ne!(second, first);
    assert!(reference.activate());
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().window_activations(),
        [first, second]
    );
}

#[test]
fn window_reference_replacement_preserves_window_identity() {
    let first = WindowRef::new();
    let second = WindowRef::new();
    let first_for_render = first.clone();
    let second_for_render = second.clone();
    let use_second = Rc::new(RefCell::new(None::<State<bool>>));
    let use_second_for_render = Rc::clone(&use_second);
    let root = component(move |cx| {
        let current = cx.use_state(|| false);
        *use_second_for_render.borrow_mut() = Some(current.clone());
        let window = Window::new("Main", text_block("main"), || {});
        Application::new([if current.get().unwrap() {
            window.reference(&second_for_render).build().key(1)
        } else {
            window.reference(&first_for_render).build().key(1)
        }])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let window = first.node().unwrap();
    assert!(!second.is_mounted());

    assert!(use_second.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert!(!first.is_mounted());
    assert_eq!(second.node(), Some(window));
    assert_eq!(reactor.engine().runtime().window_ids(), [window]);
    assert!(second.activate());
    reactor.pump();
    assert_eq!(reactor.engine().runtime().window_activations(), [window]);
}
