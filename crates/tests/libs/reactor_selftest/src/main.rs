#![windows_subsystem = "console"]

use windows_core::*;
use windows_reactor::*;

mod fixtures;
mod runner;

fn main() -> Result<()> {
    let options = match parse_options(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
    let selected = match runner::select_fixtures(options.filter.as_deref()) {
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

    if options.explicit_lifetime {
        App::run_with(move |context| {
            context.open_window(View::component::<runner::FixtureRunner>(
                runner::RunnerInput::explicit(selected, context.clone()),
            ))
        })
    } else {
        App::run_component::<runner::FixtureRunner>(runner::RunnerInput::legacy(selected))
    }
}

#[derive(Default, PartialEq, Debug)]
struct Options {
    explicit_lifetime: bool,
    filter: Option<String>,
}

fn parse_options(
    arguments: impl IntoIterator<Item = String>,
) -> std::result::Result<Options, &'static str> {
    let mut arguments = arguments.into_iter();
    let mut options = Options::default();
    while let Some(argument) = arguments.next() {
        if argument == "--filter" {
            options.filter = Some(arguments.next().ok_or("missing value for --filter")?);
        } else if let Some(value) = argument.strip_prefix("--filter=") {
            options.filter = Some(value.to_string());
        } else if argument == "--explicit-lifetime" {
            options.explicit_lifetime = true;
        }
    }

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_requires_a_value() {
        assert_eq!(
            parse_options(["--filter".to_string()]),
            Err("missing value for --filter")
        );
    }

    #[test]
    fn parses_explicit_lifetime() {
        assert_eq!(
            parse_options(["--explicit-lifetime".to_string()]),
            Ok(Options {
                explicit_lifetime: true,
                filter: None,
            })
        );
    }
}
