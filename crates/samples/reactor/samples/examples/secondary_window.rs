#![windows_subsystem = "windows"]

//! Secondary windows (#4703).
//!
//! The primary window is a launcher. Every time you press **Open counter
//! window** it opens a brand-new top-level window that hosts its own
//! independent reactor tree — each window has its own `use_state`, so the
//! counters never share a value. Windows can be closed in any order; when the
//! *last* remaining window closes (including this launcher) the app exits.

use windows_reactor::*;

/// A self-contained counter. Both secondary windows and (conceptually) any
/// other host can render this — because each window drives its own reactor
/// tree, every instance keeps a fully independent count.
fn counter_view(cx: &mut RenderCx, heading: &str) -> Element {
    let (count, set_count) = cx.use_state(0_i32);

    let dec = {
        let s = set_count.clone();
        move || s.call(count - 1)
    };
    let inc = {
        let s = set_count;
        move || s.call(count + 1)
    };

    vstack((
        text_block(heading).bold().font_size(20.0),
        text_block(format!("Count: {count}")).font_size(28.0),
        hstack((button("-").on_click(dec), button("+").on_click(inc))).spacing(8.0),
    ))
    .spacing(12.0)
    .margin(Thickness::uniform(24.0))
    .into()
}

fn app(cx: &mut RenderCx) -> Element {
    // How many secondary windows this launcher has opened so far — used only to
    // give each new window a distinct title.
    let (opened, set_opened) = cx.use_state(0_u32);

    let open = {
        let set_opened = set_opened;
        move || {
            let n = opened + 1;
            match ReactorWindow::new()
                .title(format!("Counter window #{n}"))
                .inner_size(320.0, 220.0)
                .render(move |cx| counter_view(cx, "Independent counter"))
            {
                Ok(_handle) => set_opened.call(n),
                Err(err) => eprintln!("failed to open secondary window: {err}"),
            }
        }
    };

    vstack((
        TitleBar::new("windows_reactor — secondary windows"),
        text_block("Each window you open hosts its own independent counter.").opacity(0.75),
        text_block("Closing the last remaining window exits the app.").opacity(0.75),
        button("Open counter window")
            .on_click(open)
            .automation_id("open-window-button"),
        text_block(format!("Windows opened: {opened}")).opacity(0.6),
    ))
    .spacing(12.0)
    .margin(Thickness::uniform(24.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Secondary windows", app)
}
