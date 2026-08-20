#![windows_subsystem = "console"]

use windows_core::*;
use windows_reactor_next::*;

fn main() -> Result<()> {
    bootstrap()?;
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(30));
        eprintln!("windows-reactor-next self-test timed out");
        std::process::exit(1);
    });

    App::run_windows([
        View::component::<Primary>(()),
        View::component::<Secondary>(()),
    ])?;
    Err(Error::new(
        HRESULT(0x80004005_u32 as _),
        "windows-reactor-next self-test returned before its completion marker",
    ))
}

struct Primary;

impl Component for Primary {
    type Props = ();
    type Message = String;

    fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _props: &(), _context: &mut ComponentContext<Self>) {}

    fn update(&mut self, message: String, _context: &mut ComponentContext<Self>) {
        record_live_primary_event(message);
    }

    fn view(&self, context: &mut ViewContext<Self>) -> View {
        context.use_effect((), move || {
            let passed = live_resources_installed().unwrap_or(false);
            if let Err(error) = schedule_live_controlled_repair_test(passed) {
                eprintln!("could not start live backend fixture: {error}");
                std::process::exit(1);
            }
            Some(Box::new(mark_live_test_cleanup as fn()))
        });
        let sender = context.sender();
        View::native(TextBox::new().text("fixed").on_text_changed(move |value| {
            sender.send(value);
        }))
    }
}

struct Secondary;

impl Component for Secondary {
    type Props = ();
    type Message = String;

    fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _props: &(), _context: &mut ComponentContext<Self>) {}

    fn update(&mut self, message: String, _context: &mut ComponentContext<Self>) {
        record_live_secondary_event(message);
    }

    fn view(&self, context: &mut ViewContext<Self>) -> View {
        let sender = context.sender();
        View::native(TextBox::new().text("second").on_text_changed(move |value| {
            sender.send(value);
        }))
    }
}
