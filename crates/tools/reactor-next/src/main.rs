mod generate;
mod schema;

use schema::workspace_path;
use std::fs;
use tool_reactor::metadata::MetadataResolver;

const OUTPUT: &str = "crates/libs/reactor-next/src/generated.rs";
const BINDINGS: &str = "crates/libs/reactor-next/src/native/winui/bindings.rs";
const BINDINGS_FILTER: &str = "crates/tools/reactor-next/src/bindings.txt";
const WINMD: &str = "crates/tools/reactor/winmd";
const SCHEMA: &str = "crates/tools/reactor-next/src/winui.toml";

fn main() {
    let source = fs::read_to_string(workspace_path(SCHEMA)).unwrap();
    let schema = schema::Schema::parse(&source).unwrap();
    let metadata = MetadataResolver::load(&workspace_path(WINMD));
    let resolved = schema.resolve(&metadata).unwrap();
    let generated = tool_reactor::helpers::rustfmt(&generate::generate(&resolved));

    let path = workspace_path(OUTPUT);
    if !matches!(fs::read_to_string(path).as_deref(), Ok(current) if current == generated) {
        fs::write(workspace_path(OUTPUT), generated).unwrap();
    }

    windows_bindgen::builder()
        .input(workspace_path(WINMD))
        .input_default()
        .output(workspace_path(BINDINGS))
        .minimal()
        .dead_code()
        .flat()
        .filter_file(workspace_path(BINDINGS_FILTER))
        .write();
}
