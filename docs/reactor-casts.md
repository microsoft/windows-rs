# Avoiding Unnecessary QueryInterface Casts

## The Rule

In windows-rs, every WinRT **class** implements `Deref` to its **default interface**:

```
Button → IButton, TextBlock → ITextBlock, ComboBox → IComboBox, etc.
```

Calling `.cast::<IButton>()` on a `Button` is unnecessary — it triggers a COM
`QueryInterface` at runtime when `Deref` resolves the method at zero cost.

```rust
// ❌ unnecessary QI
b.cast::<Xaml::IButton>()?.SetFlyout(&flyout)?;

// ✅ zero-cost Deref
b.SetFlyout(&flyout)?;
```

## IInspectable Conversions

All WinRT types derive from `IInspectable`. The bindings provide
`From<T> for IInspectable` (zero-cost). Never `.cast::<IInspectable>()`:

```rust
// ❌
let insp = reference.cast::<IInspectable>()?;

// ✅
let insp: IInspectable = reference.into();
```

## Param Trait — Let Methods Convert

Methods accepting `impl Param<IInspectable>` (or `UIElement`, etc.) handle
conversion automatically. Don't convert beforehand:

```rust
// ❌
let insp: IInspectable = ui_elem.into();
e.SetHeader(&insp)?;

// ✅
e.SetHeader(&ui_elem)?;
```

## Plain None

Methods with optional parameters infer the type. Never annotate `None`:

```rust
// ❌
e.SetHeader(None::<&IInspectable>);

// ✅
e.SetHeader(None);
```

## When a Cast IS Required

A cast is needed for methods on a **non-default parent interface**:

```
Button → Deref → IButton        (free)
Button → cast  → IContentControl (QI needed)
Button → cast  → IControl        (QI needed)
```

Or when a subclass needs a parent class's default interface:

```
CheckBox → cast → IToggleButton  (QI needed)
DropDownButton → cast → IButton  (QI needed)
```

Casts to generic interfaces like `IVector<IInspectable>` are also legitimate.

## How to Verify

To confirm what a class derefs to, search `bindings.rs`:

```
impl core::ops::Deref for <ClassName> {
    type Target = <IDefaultInterface>;
```

If the cast target matches `Target`, the cast is removable.
