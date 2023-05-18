use super::*;

// TODO: for the scraper we need a per-module toml file that can contain the mapping of headers and other info
// used to filter/transform the data for that module. This can be used by third parties to generate idl/metadata
// for their APIs too.

pub fn extend(_base: &mut Module, _other: Module) {
    // Adds any `new` declarations to `prev` leaving existing declarations intact.
    // May be used by tool that parses other sources, like .h and .idl files from the Windows SDK
    // to keep a canonical rs/idl style repo updated.
    // The resulting `Module` can then be written out as idl with `to_idl` and to update the repo
    // in place.
    // Maybe return a list of differences that were ignored

    todo!()
}
