mod generate;
mod schema;

use schema::workspace_path;
use std::fs;
use tool_reactor::metadata::MetadataResolver;

const OUTPUT: &str = "crates/libs/reactor-next/src/generated.rs";
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
}
