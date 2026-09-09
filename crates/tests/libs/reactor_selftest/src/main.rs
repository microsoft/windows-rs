#![windows_subsystem = "console"]

use windows_core::*;
use windows_reactor::*;

mod fixtures;
mod runner;

fn main() -> Result<()> {
    let filter = match parse_filter(std::env::args().skip(1)) {
        Ok(filter) => filter,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
    let selected = match runner::select_fixtures(filter.as_deref()) {
        Ok(selected) => selected,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    std::thread::spawn(|| {
        std::thread::sleep(runner::SUITE_TIMEOUT);
        eprintln!("windows-reactor self-test timed out");
        std::process::exit(1);
    });

    App::run_component::<runner::FixtureRunner>(selected)
}

fn parse_filter(
    arguments: impl IntoIterator<Item = String>,
) -> std::result::Result<Option<String>, &'static str> {
    let mut arguments = arguments.into_iter();
    let mut filter = None;
    while let Some(argument) = arguments.next() {
        if argument == "--filter" {
            filter = Some(arguments.next().ok_or("missing value for --filter")?);
        } else if let Some(value) = argument.strip_prefix("--filter=") {
            filter = Some(value.to_string());
        }
    }

    Ok(filter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_requires_a_value() {
        assert_eq!(
            parse_filter(["--filter".to_string()]),
            Err("missing value for --filter")
        );
    }
}
