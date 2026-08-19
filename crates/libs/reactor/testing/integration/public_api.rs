use std::time::Duration;

use windows_reactor::{
    Application, CollectionSelection, ContentDialog, ContentDialogResult, ContextKey, Element,
    ListBox, RenderCx, State, TextBox, Window, button, component, provide_context_key, stack_panel,
    text_block,
};

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_BOOTSTRAP_FIXTURE";
const UIA_FIXTURE_ENV: &str = "WINDOWS_REACTOR_UIA_FIXTURE";
const UIA_WINDOW_NAME: &str = "windows-reactor UI Automation acceptance";
const UIA_SECONDARY_WINDOW_NAME: &str = "windows-reactor UI Automation secondary";
const NATIVE_HOST_FIXTURE_ENV: &str = "WINDOWS_REACTOR_NATIVE_HOST_FIXTURE";

fn default_theme() -> String {
    "system".to_string()
}

static THEME: ContextKey<String> = ContextKey::new(default_theme);

#[test]
fn application_tree_builds_through_the_public_api() {
    let root =
        Application::new([Window::new("Test", text_block("Content"), || {}).build()]).build();

    drop(root);
}

#[test]
fn static_context_tree_builds_through_the_public_api() {
    let root = provide_context_key(
        &THEME,
        "dark".to_string(),
        component(|cx| text_block(cx.use_context_key(&THEME))),
    );

    drop(root);
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn bootstrap_runs_in_an_isolated_process() {
    let output = test_reactor_support::run_test_process(
        "bootstrap_fixture",
        &[(FIXTURE_ENV, "1")],
        Duration::from_secs(30),
    )
    .unwrap();

    test_reactor_support::assert_success(output);
}

#[test]
fn bootstrap_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    windows_reactor::bootstrap().unwrap();
}

#[test]
#[ignore = "requires the Windows App Runtime and an interactive desktop"]
fn composition_host_native_lifecycle() {
    let output = test_reactor_support::run_test_process(
        "native_host_fixture",
        &[(NATIVE_HOST_FIXTURE_ENV, "composition")],
        Duration::from_secs(30),
    )
    .unwrap_or_else(|error| panic!("{error}"));
    test_reactor_support::assert_success(output);
}

#[cfg(feature = "canvas")]
#[test]
#[ignore = "requires the Windows App Runtime, Direct2D, and an interactive desktop"]
fn swap_chain_host_native_lifecycle() {
    let output = test_reactor_support::run_test_process(
        "native_host_fixture",
        &[(NATIVE_HOST_FIXTURE_ENV, "canvas")],
        Duration::from_secs(30),
    )
    .unwrap_or_else(|error| panic!("{error}"));
    test_reactor_support::assert_success(output);
}

#[cfg(feature = "webview")]
#[test]
#[ignore = "requires the Windows App Runtime, WebView2, and an interactive desktop"]
fn webview_host_native_lifecycle() {
    let output = test_reactor_support::run_test_process(
        "native_host_fixture",
        &[(NATIVE_HOST_FIXTURE_ENV, "webview")],
        Duration::from_secs(30),
    )
    .unwrap_or_else(|error| panic!("{error}"));
    test_reactor_support::assert_success(output);
}

#[test]
fn native_host_fixture() {
    let Ok(host) = std::env::var(NATIVE_HOST_FIXTURE_ENV) else {
        return;
    };
    windows_reactor::bootstrap().unwrap();
    windows_reactor::run_reactor_winui_app(component(move |cx| {
        let open = cx.use_state(|| true);
        let close = open.clone();
        cx.use_timeout((), Duration::from_secs(3), move || close.set(false));
        cx.use_timeout((), Duration::from_secs(5), || std::process::exit(0));
        let content = native_host_content(cx, &host, open.clone());
        let windows = if open.value() {
            vec![
                Window::new("windows-reactor native host fixture", content, move || {
                    open.set(false);
                })
                .build(),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    }))
    .unwrap();
}

fn native_host_content(cx: &mut RenderCx<'_>, host: &str, _open: State<bool>) -> Element {
    match host {
        "composition" => {
            use windows_composition::SpriteVisual;
            use windows_reactor::{CompositionContent, CompositionHost};

            let reference = cx.use_composition_host_ref::<SpriteVisual>();
            CompositionHost::new(
                &reference,
                |compositor| {
                    let visual = compositor.create_sprite_visual();
                    Ok(CompositionContent::new(visual.clone(), visual))
                },
                |visual, layout| {
                    visual.set_size(layout.width, layout.height);
                    Ok(())
                },
            )
            .build()
        }
        #[cfg(feature = "canvas")]
        "canvas" => {
            use windows_canvas::{ColorF, GpuDevice};
            use windows_reactor::{SwapChainHost, SwapChainHostContent};

            let reference = cx.use_swap_chain_host_ref::<bool>();
            let update = reference.clone();
            cx.use_effect((), move || {
                assert!(update.update(|ready| {
                    *ready = true;
                    Ok(())
                }));
            });
            SwapChainHost::new(
                &reference,
                |layout| {
                    let device = GpuDevice::new_or_warp()?;
                    let swap_chain =
                        device.create_swap_chain(layout.pixel_width, layout.pixel_height)?;
                    Ok(SwapChainHostContent::new(false, swap_chain))
                },
                |_, swap_chain, layout| {
                    swap_chain.resize_with_dpi(
                        layout.pixel_width,
                        layout.pixel_height,
                        96.0 * layout.scale_x,
                        96.0 * layout.scale_y,
                    )
                },
                |ready, swap_chain, _| {
                    assert!(*ready);
                    let session = swap_chain.begin_draw()?;
                    session.clear(ColorF::DARK_SLATE_BLUE);
                    drop(session);
                    swap_chain.present().map(|_| ())
                },
            )
            .build()
        }
        #[cfg(feature = "webview")]
        "webview" => {
            use windows_reactor::WebViewHost;

            let reference = cx.use_webview_ref();
            WebViewHost::new(&reference)
                .source("about:blank")
                .on_created(move |result| {
                    result.unwrap();
                    _open.set(false);
                })
                .build()
        }
        _ => panic!("unknown native host fixture"),
    }
}

#[test]
#[ignore = "requires the Windows App Runtime and an interactive desktop"]
fn ui_automation_drives_representative_public_flows() {
    let process = test_reactor_support::TestProcess::spawn(
        "ui_automation_fixture",
        &[(UIA_FIXTURE_ENV, "1")],
    )
    .unwrap();
    let automation = test_reactor_support::Automation::new().unwrap();
    let window = automation
        .wait_for_window(process.id(), UIA_WINDOW_NAME, Duration::from_secs(30))
        .unwrap();
    let input = automation
        .wait_for_descendant_name(
            &window,
            "UI Automation controlled input",
            Duration::from_secs(10),
        )
        .unwrap();
    input.set_value("updated").unwrap();
    automation
        .wait_for_descendant_name(&window, "Input: updated", Duration::from_secs(10))
        .unwrap();

    let beta = automation
        .wait_for_descendant_name(&window, "Beta", Duration::from_secs(10))
        .unwrap();
    beta.select().unwrap();
    automation
        .wait_for_descendant_name(&window, "Selection: 20", Duration::from_secs(10))
        .unwrap();

    automation
        .wait_for_descendant_name(&window, "Open acceptance dialog", Duration::from_secs(10))
        .unwrap()
        .invoke()
        .unwrap();
    automation
        .wait_for_descendant_name(&window, "Accept", Duration::from_secs(10))
        .unwrap()
        .invoke()
        .unwrap();
    automation
        .wait_for_descendant_name(&window, "Dialog: Primary", Duration::from_secs(10))
        .unwrap();

    automation
        .wait_for_descendant_name(
            &window,
            "Open acceptance secondary window",
            Duration::from_secs(10),
        )
        .unwrap()
        .invoke()
        .unwrap();
    let secondary = automation
        .wait_for_window(
            process.id(),
            UIA_SECONDARY_WINDOW_NAME,
            Duration::from_secs(10),
        )
        .unwrap();
    automation
        .wait_for_descendant_name(
            &secondary,
            "UI Automation secondary content",
            Duration::from_secs(10),
        )
        .unwrap();

    window.close_window().unwrap();
    secondary.close_window().unwrap();
    test_reactor_support::assert_success(process.wait(Duration::from_secs(10)).unwrap());
}

#[test]
fn ui_automation_fixture() {
    if std::env::var_os(UIA_FIXTURE_ENV).is_none() {
        return;
    }

    windows_reactor::bootstrap().unwrap();
    windows_reactor::run_reactor_winui_app(component(ui_automation_application)).unwrap();
}

fn ui_automation_application(cx: &mut RenderCx) -> Element {
    let main_open = cx.use_state(|| true);
    let secondary_open = cx.use_state(|| false);
    let dialog_open = cx.use_state(|| false);
    let text = cx.use_state(|| "initial".to_string());
    let selection = cx.use_state(|| None::<u64>);
    let dialog_result = cx.use_state(|| ContentDialogResult::None);
    let current_text = text.value();
    let current_selection = selection.value();
    let current_dialog_open = dialog_open.value();
    let current_dialog_result = dialog_result.value();
    let mut windows = Vec::new();

    if main_open.value() {
        let show_secondary = secondary_open.clone();
        let show_dialog = dialog_open.clone();
        let close_dialog = dialog_open;
        let close_main = main_open;
        let update_text = text;
        let update_selection = selection;
        let update_dialog_result = dialog_result;
        let content = stack_panel([
            TextBox::new(current_text.clone(), move |value| {
                update_text.set(value);
            })
            .automation_name("UI Automation controlled input")
            .build(),
            text_block(format!("Input: {current_text}")),
            ListBox::new([(10, "Alpha"), (20, "Beta"), (30, "Gamma")], move |value| {
                update_selection.set(value.as_slice().first().copied());
            })
            .selection(CollectionSelection::new(current_selection))
            .automation_name("UI Automation collection")
            .build(),
            text_block(format!(
                "Selection: {}",
                current_selection.map_or_else(|| "none".to_string(), |key| key.to_string())
            )),
            button("Open acceptance dialog", move || {
                show_dialog.set(true);
            }),
            text_block(format!("Dialog: {current_dialog_result:?}")),
            ContentDialog::new(
                "UI Automation acceptance dialog",
                text_block("UI Automation dialog content"),
            )
            .primary_button("Accept")
            .close_button("Cancel")
            .open(current_dialog_open)
            .on_closed(move |result| {
                update_dialog_result.set(result);
                close_dialog.set(false);
            })
            .build(),
            button("Open acceptance secondary window", move || {
                show_secondary.set(true);
            }),
        ]);
        windows.push(
            Window::new(UIA_WINDOW_NAME, content, move || {
                close_main.set(false);
            })
            .build()
            .key(1),
        );
    }

    if secondary_open.value() {
        let close_secondary = secondary_open;
        windows.push(
            Window::new(
                UIA_SECONDARY_WINDOW_NAME,
                text_block("UI Automation secondary content"),
                move || {
                    close_secondary.set(false);
                },
            )
            .build()
            .key(2),
        );
    }

    Application::new(windows).build()
}
