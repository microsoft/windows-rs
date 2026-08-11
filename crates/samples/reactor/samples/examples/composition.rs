#![windows_subsystem = "windows"]

use windows_reactor::*;

fn labeled_row(label: &str, value: Element) -> Fragment {
    fragment((
        text_block(label).foreground(Color::rgb(120, 120, 120)),
        value,
    ))
}

fn badge_button(label: &str, count: u32) -> Button {
    button(format!("{label} ({count})"))
}

fn app(cx: &mut RenderCx) -> Element {
    let (inbox_count, set_inbox) = cx.use_state(3_u32);
    let (drafts_count, set_drafts) = cx.use_state(1_u32);
    let inbox_for_inc = inbox_count;
    let drafts_for_inc = drafts_count;

    vstack((
        TitleBar::new("windows_reactor - composition sample")
            .subtitle("Fragment + helper function"),
        text_block("Settings (labeled_row uses a child-only Fragment)")
            .bold()
            .font_size(20.0),
        vstack((
            labeled_row("Username", text_block("alice").into()),
            labeled_row("Theme", text_block("Dark").into()),
            labeled_row("Notifications", text_block("Enabled").into()),
        ))
        .spacing(6.0),
        text_block("Reusable widgets (badge_button composes a Button)")
            .bold()
            .font_size(20.0),
        hstack((
            badge_button("Inbox", inbox_count),
            badge_button("Drafts", drafts_count),
            button("+ Inbox").on_click(move || set_inbox.call(inbox_for_inc + 1)),
            button("+ Drafts").on_click(move || set_drafts.call(drafts_for_inc + 1)),
        ))
        .spacing(8.0),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Composition", app)
}
