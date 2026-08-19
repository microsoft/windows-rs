#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

use windows_reactor::{
    Button, CancellationToken, Element, ProgressRing, RenderCx, Resource, TextBlock, vstack,
};

fn fetch_weather(cancel: CancellationToken, attempt: i32) -> windows_core::Result<String> {
    for _ in 0..20 {
        if cancel.is_cancelled() {
            return Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004004_u32 as i32),
                "weather request cancelled",
            ));
        }
        thread::sleep(Duration::from_millis(25));
    }

    if attempt % 3 == 2 {
        Err(windows_core::Error::new(
            windows_core::HRESULT(0x80004005_u32 as i32),
            "network timeout - server unreachable",
        ))
    } else {
        Ok(format!("72 F and sunny (attempt #{})", attempt + 1))
    }
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let attempt = cx.use_state(|| 0_i32);
    let current = attempt.value();
    let weather = cx.use_resource(current, fetch_weather);

    let content = match weather {
        Resource::Loading => ProgressRing::indeterminate().build(),
        Resource::Ready(data) => TextBlock::new(data.as_str()).build(),
        Resource::Failed(error) => TextBlock::new(format!("Error: {error}")).build(),
    };

    vstack(
        8.0,
        [
            TextBlock::new("Weather Service").font_size(20.0).build(),
            TextBlock::new(format!("Attempt: {}", current + 1)).build(),
            content,
            Button::new(if current % 3 == 2 { "Retry" } else { "Refresh" })
                .on_click(move || {
                    attempt.update(|value| *value += 1);
                })
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("UseResourceRetry", app)
}
