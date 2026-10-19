//! The Rust component that fixture clients activate. Each fixture's optional `component.rs` is a
//! module of this crate that implements the fixture's API. See `readme.md`.

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    include!(concat!(env!("OUT_DIR"), "/win32.rs"));
}

use bindings::*;
use windows_core::*;

include!(concat!(env!("OUT_DIR"), "/component.rs"));

#[unsafe(no_mangle)]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    result: OutRef<IActivationFactory>,
) -> HRESULT {
    let factory = fixture_factory(&name.to_string_lossy());
    let code = if factory.is_some() {
        S_OK
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    };
    _ = result.write(factory);
    code
}

#[implement(IActivationFactory)]
struct Activatable(fn() -> IInspectable);

impl IActivationFactory_Impl for Activatable_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok((self.0)())
    }
}

/// Returns a factory that activates `T::default()` as the class `C`.
///
/// The instance is returned through the class's default interface rather than a separate
/// `IInspectable` tear-off. `IActivationFactory` does not require this, but CsWinRT 3 calls the
/// returned pointer's default interface methods without a `QueryInterface`.
fn activatable<T: Default + Into<C>, C: Into<IInspectable>>() -> IActivationFactory {
    Activatable(|| {
        let instance: C = T::default().into();
        instance.into()
    })
    .into()
}
