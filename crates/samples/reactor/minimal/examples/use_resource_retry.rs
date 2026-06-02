//! Demonstrates `use_resource` with error handling and retry.
//!
//! Simulates a flaky API that fails every 3rd attempt. Shows
//! loading/error/ready states and retry by bumping deps.

use std::thread;
use std::time::Duration;

use windows_reactor::*;

fn fetch_weather(attempt: i32) -> std::result::Result<String, String> {
    thread::sleep(Duration::from_millis(600));
    if attempt % 3 == 2 {
        Err("Network timeout - server unreachable".to_string())
    } else {
        Ok(format!("72F Sunny (attempt #{})", attempt + 1))
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let (attempt, set_attempt) = cx.use_state(0_i32);

    let weather = cx.use_resource(fetch_weather, attempt);

    let retry = {
        let set_attempt = set_attempt;
        move || set_attempt.call(attempt + 1)
    };

    let content: Element = match &weather {
        Resource::Loading => ProgressRing::indeterminate().into(),
        Resource::Ready(data) => text_block(data).into(),
        Resource::Error(e) => vstack((
            text_block(format!("Error: {e}")),
            button("Retry").on_click(retry.clone()),
        ))
        .into(),
        _ => text_block("...").into(),
    };

    vstack((
        text_block("Weather Service (flaky API demo)").font_size(20.0),
        text_block(format!("Attempt: {}", attempt + 1)),
        content,
        button("Refresh").on_click(retry),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("use_resource_retry").render(app)
}
