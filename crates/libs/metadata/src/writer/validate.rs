use super::*;

pub fn validate(_module: &Module) -> Result<()> {
    // TODO:
    // *. if a TypeDef is WinRT then there shall only be one for a given name - MIDLRT may be produce more but that should be removed before `validate` is run.
    // *. if a TypeDef is not WinRT then there can be more - check that they have non-overlapping supported architectures.
    // *.
    Ok(())
}
