## The implement macro for the Windows crates

See [windows-core](https://crates.io/crates/windows-core) for more information.

### Note on `_Impl` traits and the inner/outer type

User-facing `_Impl` traits (e.g. `IFoo_Impl`) are implemented on the user's own value
type (`impl IFoo_Impl for MyApp`), not on the macro-synthesized outer wrapper
(`MyApp_Impl`). This reverses the convention introduced by
[#3065](https://github.com/microsoft/windows-rs/pull/3065), which moved the
implementations to the outer type so that methods could access the full COM object
(vtables, reference count, identity) — for example, to query for another interface
that the same object implements and return it as an output parameter.

That capability is preserved: vtable thunks now bind `<Identity as IUnknownImpl>::Impl:
IFoo_Impl` and dispatch through `Identity::get_impl(outer)`, and the
`windows_core::IUnknownImpl` / `windows_core::ComObjectInner` traits provide default
helpers — `outer()`, `to_object()`, `as_interface::<I>()`, and `to_interface::<I>()` —
that recover the outer wrapper from `&self` and return interface pointers or
`ComObject<Self>`. So `self.to_interface::<IFoo>()` inside an `_Impl` method achieves
the same result as `self.cast::<IFoo>()` did under the #3065 design, while keeping
the user's type definition unchanged and avoiding the `MyApp_Impl` name at use sites.
These helpers are sound as long as the `&self` reference was obtained through a
`ComObject<MyApp>` (i.e. through the normal vtable dispatch path), which is the only
way safe Rust code receives such a reference in practice.
