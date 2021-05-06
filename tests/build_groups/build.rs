fn main() {
    windows::build!(
        // TODO: IKeyValuePair should not need to be needed here https://github.com/microsoft/windows-rs/issues/772
        Windows::Foundation::{IStringable, Collections::{IVector, IMap, IKeyValuePair}},
    );
}
