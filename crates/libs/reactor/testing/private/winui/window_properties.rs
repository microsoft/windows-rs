use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::*;
use crate::winui::window::tests as window_probe;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_WINDOW_PROPERTY_FIXTURE";

fn run_case(case: &str) {
    let output = test_reactor_support::run_test_process(
        "winui::tests::window_properties::window_property_fixture",
        &[(FIXTURE_ENV, case)],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn icon_applies() {
    run_case("icon");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn backdrop_applies() {
    run_case("backdrop");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn theme_survives_content_replacement_and_updates() {
    run_case("theme");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn title_bar_updates_and_resets() {
    run_case("title-bar");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn overlapped_policy_updates_and_resets() {
    run_case("overlapped");
}

#[test]
fn window_property_fixture() {
    let Some(case) = std::env::var_os(FIXTURE_ENV) else {
        return;
    };

    bootstrap().unwrap();
    match case.to_str().unwrap() {
        "icon" => icon_fixture(),
        "backdrop" => backdrop_fixture(),
        "theme" => theme_fixture(),
        "title-bar" => title_bar_fixture(),
        "overlapped" => overlapped_fixture(),
        case => panic!("unknown window property fixture: {case}"),
    }
}

fn icon_fixture() {
    let icon = std::fs::canonicalize(concat!(
        env!("CARGO_MANIFEST_DIR"),
        r"\..\..\samples\reactor\samples\examples\icon.ico"
    ))
    .unwrap();
    let icon = icon.to_str().unwrap();
    let icon = WindowIcon::file(icon.strip_prefix(r"\\?\").unwrap_or(icon));
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let render_icon = icon.clone();
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        publish_open_state.borrow_mut().replace(open.clone());
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new("Icon window", text_block("Icon content"), move || {
                    open.set(false);
                })
                .icon(render_icon.clone())
                .build()
                .key(1),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let runtime = reactor.engine().runtime();
        let windows: Vec<_> = RuntimeProbe::new(runtime).windows().collect();
        assert_eq!(windows.len(), 1);
        assert_eq!(window_probe::icon(runtime, windows[0])?, Some(icon.clone()));
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        assert!(
            RuntimeProbe::new(reactor.engine().runtime())
                .windows()
                .next()
                .is_none()
        );
        Ok(())
    })
    .unwrap();
}

fn backdrop_fixture() {
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        publish_open_state.borrow_mut().replace(open.clone());
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new(
                    "Backdrop window",
                    text_block("Backdrop content"),
                    move || {
                        open.set(false);
                    },
                )
                .backdrop(WindowBackdrop::MicaAlt)
                .build()
                .key(1),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let runtime = reactor.engine().runtime();
        let windows: Vec<_> = RuntimeProbe::new(runtime).windows().collect();
        assert_eq!(windows.len(), 1);
        assert_eq!(
            window_probe::backdrop(runtime, windows[0])?,
            Some(WindowBackdrop::MicaAlt)
        );
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        assert!(
            RuntimeProbe::new(reactor.engine().runtime())
                .windows()
                .next()
                .is_none()
        );
        Ok(())
    })
    .unwrap();
}

fn theme_fixture() {
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let alternate_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_alternate_state = Rc::clone(&alternate_state);
    let theme_state = Rc::new(RefCell::new(None::<State<WindowTheme>>));
    let publish_theme_state = Rc::clone(&theme_state);
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        let alternate = cx.use_state(|| false);
        let theme = cx.use_state(|| WindowTheme::Dark);
        publish_open_state.borrow_mut().replace(open.clone());
        publish_alternate_state
            .borrow_mut()
            .replace(alternate.clone());
        publish_theme_state.borrow_mut().replace(theme.clone());
        let content = if alternate.try_value().unwrap() {
            TextBlock::new("Replacement").build()
        } else {
            text_block("Initial")
        };
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new("Theme window", content, move || {
                    open.set(false);
                })
                .theme(theme.try_value().unwrap())
                .build()
                .key(1),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let window = RuntimeProbe::new(reactor.engine().runtime())
            .windows()
            .next()
            .unwrap();
        assert_eq!(
            window_probe::theme(reactor.engine().runtime(), window)?,
            WindowTheme::Dark
        );
        assert!(alternate_state.borrow().as_ref().unwrap().try_set(true));
        reactor.pump();
        assert_eq!(
            window_probe::theme(reactor.engine().runtime(), window)?,
            WindowTheme::Dark
        );
        assert!(
            theme_state
                .borrow()
                .as_ref()
                .unwrap()
                .try_set(WindowTheme::Light)
        );
        reactor.pump();
        assert_eq!(
            window_probe::theme(reactor.engine().runtime(), window)?,
            WindowTheme::Light
        );
        assert!(
            theme_state
                .borrow()
                .as_ref()
                .unwrap()
                .try_set(WindowTheme::System)
        );
        reactor.pump();
        assert_eq!(
            window_probe::theme(reactor.engine().runtime(), window)?,
            WindowTheme::System
        );
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn title_bar_fixture() {
    let first = SystemTitleBar {
        extend_content: true,
        height: TitleBarHeight::Tall,
        icon: SystemTitleBarIconPolicy::HideIconAndSystemMenu,
        colors: SystemTitleBarColors {
            foreground: Some(Color::rgb(255, 255, 255)),
            background: Some(Color::rgb(20, 40, 60)),
            button_foreground: Some(Color::rgb(255, 255, 255)),
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
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        let phase = cx.use_state(|| 0usize);
        publish_open_state.borrow_mut().replace(open.clone());
        publish_phase_state.borrow_mut().replace(phase.clone());
        let close = open.clone();
        let window = Window::new(
            "Title-bar window",
            text_block("Title-bar content"),
            move || {
                close.set(false);
            },
        );
        let window = match phase.try_value().unwrap() {
            0 => window.title_bar(first),
            1 => window.title_bar(second),
            _ => window.presenter(WindowPresenter::FullScreen),
        };
        Application::new(if open.try_value().unwrap() {
            vec![window.build().key(1)]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let window = RuntimeProbe::new(reactor.engine().runtime())
            .windows()
            .next()
            .unwrap();
        assert_eq!(
            window_probe::title_bar(reactor.engine().runtime(), window)?,
            first
        );
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_eq!(
            window_probe::title_bar(reactor.engine().runtime(), window)?,
            second
        );
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_eq!(
            window_probe::title_bar(reactor.engine().runtime(), window)?,
            SystemTitleBar::default()
        );
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn overlapped_fixture() {
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
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        let phase = cx.use_state(|| 0usize);
        publish_open_state.borrow_mut().replace(open.clone());
        publish_phase_state.borrow_mut().replace(phase.clone());
        let close = open.clone();
        let window = Window::new(
            "Overlapped window",
            text_block("Overlapped content"),
            move || {
                close.set(false);
            },
        );
        let window = match phase.try_value().unwrap() {
            0 => window.overlapped(first),
            1 => window.overlapped(second),
            _ => window.presenter(WindowPresenter::CompactOverlay),
        };
        Application::new(if open.try_value().unwrap() {
            vec![window.build().key(1)]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let window = RuntimeProbe::new(reactor.engine().runtime())
            .windows()
            .next()
            .unwrap();
        assert_eq!(
            window_probe::overlapped(reactor.engine().runtime(), window)?,
            first
        );
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_eq!(
            window_probe::overlapped(reactor.engine().runtime(), window)?,
            second
        );
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_eq!(
            window_probe::overlapped(reactor.engine().runtime(), window)?,
            WindowOverlappedPolicy::default()
        );
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}
