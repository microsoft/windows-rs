use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::media_access as media_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_MEDIA_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn icon_elements_update_native_properties() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::media_properties::media_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn media_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let root = component(move |cx| {
        let phase = cx.use_state(|| false);
        publish_phase_state.borrow_mut().replace(phase.clone());
        let changed = phase.value();
        StackPanel::new([
            Button::new("Symbol")
                .icon(Icon::symbol(if changed {
                    IconSymbol::SAVE
                } else {
                    IconSymbol::FAVORITE
                }))
                .build(),
            Button::new("Font")
                .icon(Icon::font(
                    if changed { "\u{E105}" } else { "\u{E113}" },
                    "Segoe Fluent Icons",
                ))
                .build(),
            Button::new("Bitmap")
                .icon(Icon::bitmap(
                    if changed {
                        "ms-appx:///second.png"
                    } else {
                        "ms-appx:///first.png"
                    },
                    changed,
                ))
                .build(),
            Button::new("Image")
                .icon(Icon::image(ImageSource::svg("ms-appx:///image.svg")))
                .build(),
            Button::new("Path")
                .icon(Icon::path(if changed {
                    "F1 M 0,8 L 8,0 L 16,8 L 8,16 Z"
                } else {
                    "F1 M 0,8 L 6,14 L 16,2 L 14,0 L 6,10 L 2,6 Z"
                }))
                .build(),
        ])
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let before = assert_icons(reactor.engine().runtime(), false);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(true));
        reactor.pump();
        let after = assert_icons(reactor.engine().runtime(), true);
        assert_eq!(before.0, after.0);
        assert_ne!(before.1, after.1);
        Ok(())
    })
    .unwrap();
}

fn assert_icons(runtime: &WinUiRuntime, changed: bool) -> (NodeId, (usize, usize)) {
    let probe = RuntimeProbe::new(runtime);
    let symbol = probe.nodes(NativeKind::SymbolIcon);
    let font = probe.nodes(NativeKind::FontIcon);
    let bitmap = probe.nodes(NativeKind::BitmapIcon);
    let image = probe.nodes(NativeKind::ImageIcon);
    let path = probe.nodes(NativeKind::PathIcon);
    assert_eq!(
        (
            symbol.len(),
            font.len(),
            bitmap.len(),
            image.len(),
            path.len()
        ),
        (1, 1, 1, 1, 1)
    );
    assert_eq!(
        media_probe::symbol(runtime, symbol[0]).unwrap(),
        if changed {
            IconSymbol::SAVE.value()
        } else {
            IconSymbol::FAVORITE.value()
        }
    );
    assert_eq!(
        media_probe::font(runtime, font[0]).unwrap(),
        (
            if changed { "\u{E105}" } else { "\u{E113}" }.into(),
            "Segoe Fluent Icons".into(),
        )
    );
    let bitmap = media_probe::bitmap(runtime, bitmap[0]).unwrap();
    assert!(
        bitmap
            .0
            .ends_with(if changed { "second.png" } else { "first.png" })
    );
    assert_eq!(bitmap.1, changed);
    assert_eq!(
        media_probe::image_size(runtime, image[0]).unwrap(),
        (20.0, 20.0)
    );
    (
        path[0],
        media_probe::path_identity(runtime, path[0]).unwrap(),
    )
}
