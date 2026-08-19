use std::rc::Rc;

use crate::element::KeyboardAccelerator;
use crate::engine::Engine;
use crate::framework_properties::{DropHandler, FrameworkProps, PointerHandlers};
use crate::hooks::TimerInvocation;
use crate::id::NodeId;
use crate::mounted::MountedKind;
use crate::runtime::{
    KeyboardAcceleratorSpec, NativeEvent, NativeRuntime, PointerEventKind, PointerEvents,
    PointerSubscription,
};

pub(crate) fn dispatch_native_event<R: NativeRuntime>(engine: &Engine<R>, event: NativeEvent) {
    match event {
        NativeEvent::TimerFired {
            owner,
            slot,
            revision,
        } => {
            let callback = mounted_kind(engine, owner).and_then(|kind| match kind {
                MountedKind::Component { hooks, .. } => hooks
                    .get(slot as usize)
                    .and_then(|hook| hook.take_timer_callback(revision)),
                _ => None,
            });
            if let Some(callback) = callback {
                match callback {
                    TimerInvocation::Once(callback) => callback(),
                    TimerInvocation::Repeating(callback) => callback(),
                }
            }
        }
        NativeEvent::WindowCloseRequested { target } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::Window(window) => Some(Rc::clone(&window.props.on_close_requested)),
                _ => None,
            });
            if let Some(handler) = handler {
                handler();
            }
        }
        NativeEvent::WindowSizeChanged { target, size } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::Window(window) => Some(Rc::clone(&window.props.on_size_changed)),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(size);
            }
        }
        NativeEvent::WindowColorSchemeChanged { target, scheme } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::Window(window) => {
                    Some(Rc::clone(&window.props.on_color_scheme_changed))
                }
                _ => None,
            });
            if let Some(handler) = handler {
                handler(scheme);
            }
        }
        NativeEvent::Click { target } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::Button(props) => props.on_click.as_ref().map(Rc::clone),
                MountedKind::ButtonEvent(handler) => handler.as_ref().map(Rc::clone),
                MountedKind::SplitButton(props) => props.on_click.as_ref().map(Rc::clone),
                MountedKind::SplitButtonEvent(handler) => handler.as_ref().map(Rc::clone),
                MountedKind::HyperlinkButton(props) => props.on_click.as_ref().map(Rc::clone),
                MountedKind::RepeatButton(props) => props.on_click.as_ref().map(Rc::clone),
                MountedKind::ToggleButton(props) => props.on_click.as_ref().map(Rc::clone),
                MountedKind::AppBarButton(props) => Some(Rc::clone(&props.on_click)),
                _ => None,
            });
            if let Some(handler) = handler {
                handler();
            }
        }
        NativeEvent::MenuItemClick { target, key } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::MenuBar(props) => props.handlers.get(&key).map(Rc::clone),
                MountedKind::MenuFlyout(props) => props.handlers.get(&key).map(Rc::clone),
                _ => None,
            });
            if let Some(handler) = handler {
                handler();
            }
        }
        NativeEvent::TextChanged { target, value } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::TextBox(props) => props.on_change.as_ref().map(Rc::clone),
                MountedKind::RichEditBox(props) => props.on_change.as_ref().map(Rc::clone),
                MountedKind::AutoSuggestBox(props) => props.on_text_changed.as_ref().map(Rc::clone),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::PasswordChanged { target, value } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::PasswordBox(props) => props.on_change.as_ref().map(Rc::clone),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::Toggled { target, value } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::CheckBox(props) => props.on_toggle.as_ref().map(Rc::clone),
                MountedKind::RadioButton(props) => props.on_toggle.as_ref().map(Rc::clone),
                MountedKind::ToggleButton(props) => props.on_toggle.as_ref().map(Rc::clone),
                MountedKind::ToggleSwitch(props) => props.on_toggle.as_ref().map(Rc::clone),
                MountedKind::AppBarToggleButton(props) => Some(Rc::clone(&props.on_toggled)),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::ValueChanged { target, value } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::Slider(props) => props.on_change.as_ref().map(Rc::clone),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::OptionalValueChanged { target, value } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::NumberBox(props) => props.on_change.as_ref().map(Rc::clone),
                MountedKind::RatingControl(props) => props.on_change.as_ref().map(Rc::clone),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::ColorChanged { target, value } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::ColorPicker(props) => props.on_change.as_ref().map(Rc::clone),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::DateChanged { target, value } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::DatePicker(props) => props.on_change.as_ref().map(Rc::clone),
                MountedKind::CalendarDatePicker(props) => props.on_change.as_ref().map(Rc::clone),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::TimeChanged { target, value } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::TimePicker(props) => props.on_change.as_ref().map(Rc::clone),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::DatesChanged { target, value } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::CalendarView(props) => props.on_change.as_ref().map(Rc::clone),
                _ => None,
            });
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::KeyboardAcceleratorInvoked {
            target,
            accelerator,
        } => {
            let handler = mounted_kind(engine, target)
                .and_then(mounted_framework_props)
                .and_then(|props| {
                    props
                        .keyboard_accelerators()
                        .iter()
                        .find(|value| accelerator_spec(value) == accelerator)
                        .cloned()
                });
            if let Some(handler) = handler {
                handler.invoke();
            }
        }
        NativeEvent::Pointer {
            target,
            kind,
            event,
        } => {
            let handler = mounted_kind(engine, target)
                .and_then(mounted_framework_props)
                .and_then(FrameworkProps::pointer_handlers)
                .and_then(|handlers| pointer_handler(handlers, kind))
                .cloned();
            if let Some(handler) = handler {
                handler.call(event);
            }
        }
        NativeEvent::Tapped { target } => {
            let handler = mounted_kind(engine, target)
                .and_then(mounted_framework_props)
                .and_then(FrameworkProps::pointer_handlers)
                .and_then(|handlers| handlers.tapped.as_ref())
                .cloned();
            if let Some(handler) = handler {
                handler.call(());
            }
        }
        NativeEvent::RightTapped { target } => {
            let handler = mounted_kind(engine, target)
                .and_then(mounted_framework_props)
                .and_then(FrameworkProps::pointer_handlers)
                .and_then(|handlers| handlers.right_tapped.as_ref())
                .cloned();
            if let Some(handler) = handler {
                handler.call(());
            }
        }
        NativeEvent::Drop { target, result } => {
            let handler = mounted_kind(engine, target)
                .and_then(mounted_framework_props)
                .and_then(FrameworkProps::drop_handler)
                .map(DropHandler::callback)
                .cloned();
            if let Some(handler) = handler {
                handler.call(*result);
            }
        }
        NativeEvent::Scroll { target, event } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::ScrollViewer(props) => props.on_view_changed.as_ref(),
                    MountedKind::ScrollView(props) => props.on_view_changed.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler.call(event);
            }
        }
        NativeEvent::PaneClosed { target } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::SplitView(props) => props.on_pane_closed.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler.call(());
            }
        }
        NativeEvent::NavigationPaneOpenChanged { target, open } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::NavigationView(props) => props.on_pane_open_changed.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(open);
            }
        }
        NativeEvent::NavigationDisplayModeChanged { target, mode } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::NavigationView(props) => props.on_display_mode_changed.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(mode);
            }
        }
        NativeEvent::ExpandedChanged { target, expanded } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::Expander(props) => props.on_expanded_changed.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler.call(expanded);
            }
        }
        NativeEvent::TreeNodeExpandedChanged {
            target,
            key,
            expanded,
        } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::TreeView(props) => props.on_expanded_changed.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(key, expanded);
            }
        }
        NativeEvent::TeachingTipClosed { target } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::TeachingTip(props) => props.on_closed.clone(),
                _ => None,
            });
            if let Some(handler) = handler {
                handler.call(());
            }
        }
        NativeEvent::TeachingTipAction { target } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::TeachingTip(props) => props.on_action_button_click.clone(),
                _ => None,
            });
            if let Some(handler) = handler {
                handler.call(());
            }
        }
        NativeEvent::InfoBarCloseRequested { target } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::InfoBar(props) => props.on_close_requested.clone(),
                _ => None,
            });
            if let Some(handler) = handler {
                handler();
            }
        }
        NativeEvent::TitleBarBackRequested { target } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::TitleBar(props) => Some(props.on_back_requested.clone()),
                _ => None,
            });
            if let Some(handler) = handler {
                handler();
            }
        }
        NativeEvent::TitleBarPaneRequested { target } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::TitleBar(props) => Some(props.on_pane_requested.clone()),
                _ => None,
            });
            if let Some(handler) = handler {
                handler();
            }
        }
        NativeEvent::FlyoutOpened { target } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::Flyout(props) => props.on_opened.clone(),
                MountedKind::MenuFlyout(props) => props.on_opened.clone(),
                MountedKind::CommandBarFlyout(props) => props.on_opened.clone(),
                _ => None,
            });
            if let Some(handler) = handler {
                handler();
            }
        }
        NativeEvent::FlyoutClosed { target } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::Flyout(props) => props.on_closed.clone(),
                MountedKind::MenuFlyout(props) => props.on_closed.clone(),
                MountedKind::CommandBarFlyout(props) => props.on_closed.clone(),
                _ => None,
            });
            if let Some(handler) = handler {
                handler();
            }
        }
        NativeEvent::ContentDialogClosed { target, result } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::ContentDialog(props) => props.on_closed.clone(),
                _ => None,
            });
            if let Some(handler) = handler {
                handler.call(result);
            }
        }
        NativeEvent::ImageLoad {
            target,
            source_revision,
            result,
        } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::Image {
                    props,
                    source_revision: current_revision,
                } if *current_revision == source_revision => props.on_load.clone(),
                _ => None,
            });
            if let Some(handler) = handler {
                handler.call(result);
            }
        }
        NativeEvent::DeferredReady { .. } => unreachable!(),
        NativeEvent::ItemInvoked { target, key } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::VirtualCollection(props) => props.on_item_invoked.as_ref(),
                    MountedKind::BreadcrumbBar(props) => props.on_item_clicked.as_ref(),
                    MountedKind::AutoSuggestBox(props) => props.on_suggestion_chosen.as_ref(),
                    MountedKind::TreeView(props) => props.on_item_invoked.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(key);
            }
        }
        NativeEvent::QuerySubmitted { target, value } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::AutoSuggestBox(props) => props.on_query_submitted.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(value);
            }
        }
        NativeEvent::SelectionChanged { target, selection } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::VirtualCollection(props) => props.on_selection_changed.as_ref(),
                    MountedKind::ListBox(props) => props.on_selection_changed.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(selection);
            }
        }
        NativeEvent::IndexChanged { target, index } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::FlipView(props) => props.on_selection_changed.as_ref(),
                    MountedKind::TabView(props) => props.on_selection_changed.as_ref(),
                    MountedKind::Pivot(props) => props.on_selection_changed.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(index);
            }
        }
        NativeEvent::TabCloseRequested { target, key } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::TabView(props) => props.on_close_requested.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(key);
            }
        }
        NativeEvent::AddTabButtonClick { target } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::TabView(props) => props.on_add_tab_button_click.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler();
            }
        }
        NativeEvent::TabsReordered { target, keys } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::TabView(props) => props.on_tabs_reordered.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(keys);
            }
        }
        NativeEvent::ItemsReordered { target, keys } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::VirtualCollection(props) => props.on_items_reordered.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(keys);
            }
        }
        NativeEvent::SelectedKeyChanged { target, key } => {
            let handler = mounted_kind(engine, target)
                .and_then(|kind| match kind {
                    MountedKind::SelectorBar(props) => props.on_selection_changed.as_ref(),
                    MountedKind::ComboBox(props) => props.on_selection_changed.as_ref(),
                    MountedKind::RadioButtons(props) => props.on_selection_changed.as_ref(),
                    MountedKind::NavigationView(props) => props.on_selection_changed.as_ref(),
                    _ => None,
                })
                .cloned();
            if let Some(handler) = handler {
                handler(key);
            }
        }
        #[cfg(feature = "webview")]
        NativeEvent::WebViewCreated { target, result } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::WebViewHost(props) => props.on_created.clone(),
                _ => None,
            });
            if let Some(handler) = handler {
                handler.call(result);
            }
        }
        #[cfg(feature = "webview")]
        NativeEvent::WebViewNavigationCompleted {
            target,
            navigation_id,
            is_success,
            source,
        } => {
            let handler = mounted_kind(engine, target).and_then(|kind| match kind {
                MountedKind::WebViewHost(props) => props.on_navigation_completed.clone(),
                _ => None,
            });
            if let Some(handler) = handler {
                handler.call(crate::webview::WebViewNavigationCompleted {
                    navigation_id,
                    is_success,
                    source: source.into(),
                });
            }
        }
        NativeEvent::Realize { .. } | NativeEvent::Recycle { .. } => unreachable!(),
        NativeEvent::CompositionLayout { .. } => unreachable!(),
        #[cfg(feature = "webview")]
        NativeEvent::WebViewInitializationReady { .. } => unreachable!(),
        #[cfg(feature = "canvas")]
        NativeEvent::CanvasImageLayout { .. }
        | NativeEvent::CanvasImageFrame { .. }
        | NativeEvent::CanvasLayout { .. }
        | NativeEvent::CanvasFrame { .. }
        | NativeEvent::SwapChainHostLayout { .. }
        | NativeEvent::SwapChainHostFrame { .. } => unreachable!(),
    }
}

fn mounted_kind<R: NativeRuntime>(engine: &Engine<R>, id: NodeId) -> Option<&MountedKind> {
    engine
        .arena
        .get(id)
        .and_then(|node| node.mounted.as_ref())
        .map(|mounted| &mounted.kind)
}

pub(crate) fn accelerator_spec(value: &KeyboardAccelerator) -> KeyboardAcceleratorSpec {
    KeyboardAcceleratorSpec {
        key: value.key(),
        modifiers: value.modifiers(),
    }
}

fn pointer_handler(
    handlers: &PointerHandlers,
    kind: PointerEventKind,
) -> Option<&crate::element::Callback<crate::element::PointerEvent>> {
    match kind {
        PointerEventKind::Pressed => handlers.pressed.as_ref(),
        PointerEventKind::Moved => handlers.moved.as_ref(),
        PointerEventKind::Released => handlers.released.as_ref(),
        PointerEventKind::CaptureLost => handlers.capture_lost.as_ref(),
        PointerEventKind::Canceled => handlers.canceled.as_ref(),
        PointerEventKind::Entered => handlers.entered.as_ref(),
        PointerEventKind::Exited => handlers.exited.as_ref(),
    }
}

fn mounted_framework_props(kind: &MountedKind) -> Option<&FrameworkProps> {
    kind.framework_props()
}

pub(crate) fn pointer_subscription(handlers: Option<&PointerHandlers>) -> PointerSubscription {
    let Some(handlers) = handlers else {
        return PointerSubscription::default();
    };
    let mut events = PointerEvents::default();
    if handlers.pressed.is_some() || handlers.capture_on_press {
        events |= PointerEvents::PRESSED;
    }
    if handlers.moved.is_some() {
        events |= PointerEvents::MOVED;
    }
    if handlers.released.is_some() || handlers.capture_on_press {
        events |= PointerEvents::RELEASED;
    }
    if handlers.capture_lost.is_some() || handlers.capture_on_press {
        events |= PointerEvents::CAPTURE_LOST;
    }
    if handlers.canceled.is_some() || handlers.capture_on_press {
        events |= PointerEvents::CANCELED;
    }
    if handlers.entered.is_some() {
        events |= PointerEvents::ENTERED;
    }
    if handlers.exited.is_some() {
        events |= PointerEvents::EXITED;
    }
    if handlers.tapped.is_some() {
        events |= PointerEvents::TAPPED;
    }
    if handlers.right_tapped.is_some() {
        events |= PointerEvents::RIGHT_TAPPED;
    }
    PointerSubscription {
        events,
        capture_on_press: handlers.capture_on_press,
    }
}
