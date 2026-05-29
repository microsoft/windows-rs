use crate::controls::*;
use windows_reactor::*;

pub fn toggle_switch_page(_: &(), cx: &mut RenderCx) -> Element {
    let (wifi_on, set_wifi_on) = cx.use_state(true);
    let (notifications_on, set_notifications_on) = cx.use_state(false);
    let (automation_enabled, set_automation_enabled) = cx.use_state(false);
    let (overnight_updates, set_overnight_updates) = cx.use_state(true);

    page_content(
        "ToggleSwitch",
        "A compact switch for turning a setting on or off with immediate feedback.",
        vec![
            sample_card(
                "Basic Toggle",
                vstack((
                    ToggleSwitch::new(wifi_on)
                        .on_content("On")
                        .off_content("Off")
                        .on_changed({
                            let set_wifi_on = set_wifi_on;
                            move |value| set_wifi_on.call(value)
                        }),
                    text_block(if wifi_on {
                        "Wi-Fi is connected."
                    } else {
                        "Wi-Fi is turned off."
                    })
                    .opacity(0.6),
                ))
                .spacing(8.0),
                r#"ToggleSwitch::new(wifi_on)
    .on_changed(...)"#,
            ),
            sample_card(
                "Toggle with Header",
                vstack((
                    ToggleSwitch::new(notifications_on)
                        .header("Notifications")
                        .on_content("On")
                        .off_content("Muted")
                        .on_changed({
                            let set_notifications_on = set_notifications_on;
                            move |value| set_notifications_on.call(value)
                        }),
                    text_block(if notifications_on {
                        "Priority alerts will appear on the desktop."
                    } else {
                        "Only badges and summaries will be shown."
                    })
                    .opacity(0.6),
                ))
                .spacing(8.0),
                r#"ToggleSwitch::new(notifications_on)
    .header("Notifications")
    .on_changed(...)"#,
            ),
            sample_card(
                "Disabled State",
                vstack((
                    ToggleSwitch::new(automation_enabled)
                        .header("Enable scheduled updates")
                        .on_changed({
                            let set_automation_enabled = set_automation_enabled;
                            move |value| set_automation_enabled.call(value)
                        }),
                    ToggleSwitch::new(overnight_updates)
                        .header("Install updates overnight")
                        .enabled(automation_enabled)
                        .on_changed({
                            let set_overnight_updates = set_overnight_updates;
                            move |value| set_overnight_updates.call(value)
                        }),
                    text_block(if !automation_enabled {
                        "Turn on scheduled updates to configure this option."
                    } else if overnight_updates {
                        "Updates will install automatically at 2:00 AM."
                    } else {
                        "Updates will wait for manual approval."
                    })
                    .opacity(0.6),
                ))
                .spacing(8.0),
                r#"ToggleSwitch::new(overnight_updates)
    .enabled(automation_enabled)
    .on_changed(...)"#,
            ),
        ],
    )
    .into()
}
