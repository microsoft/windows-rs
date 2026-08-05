#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;

use windows::{Win32::*, core::*};

#[unsafe(no_mangle)]
extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "Bench.Widget" {
        factory.write(Some(CLASS_FACTORY.to_interface())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

static CLASS_FACTORY: StaticComObject<ClassFactory> = ClassFactory.into_static();

#[implement(IActivationFactory)]
struct ClassFactory;

impl IActivationFactory_Impl for ClassFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        // Return the default interface (IWidget) as IInspectable, keeping its vtable, rather
        // than a bare IInspectable tear-off. Projections such as CsWinRT call the returned
        // pointer's interface methods without re-querying, so it must carry the IWidget slots.
        let widget: bindings::Widget = Widget::default().into();
        Ok(widget.into())
    }
}

// Process-wide count of live Widget instances. Constructing a Widget increments it; dropping the
// implementation object (when its COM refcount reaches zero) decrements it. A consumer reads it
// through `LiveCount` to prove every projection balances its AddRef/Release the way Rust's Drop
// does: after activating, casting, and disposing N objects, the count must return to its baseline.
static LIVE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

#[implement(bindings::Widget, bindings::INonDefault)]
struct Widget {
    value: std::sync::atomic::AtomicI32,
    // Registered event handlers paired with the token returned to the subscriber. A native call
    // into `Signal` invokes each one, driving the reverse-vtable path a projected delegate builds.
    handlers: std::sync::Mutex<Vec<(i64, bindings::ChangedHandler)>>,
    next_token: std::sync::atomic::AtomicI64,
}

impl Default for Widget {
    fn default() -> Self {
        LIVE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            value: std::sync::atomic::AtomicI32::new(0),
            handlers: std::sync::Mutex::new(Vec::new()),
            next_token: std::sync::atomic::AtomicI64::new(1),
        }
    }
}

impl Drop for Widget {
    fn drop(&mut self) {
        LIVE.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
}

impl bindings::IWidget_Impl for Widget_Impl {
    fn Int32Property(&self) -> Result<i32> {
        Ok(self.value.load(std::sync::atomic::Ordering::Relaxed))
    }

    fn SetInt32Property(&self, value: i32) -> Result<()> {
        self.value
            .store(value, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    fn StringProperty(&self) -> Result<HSTRING> {
        Ok(h!("widget").to_owned())
    }

    fn SetStringProperty(&self, _value: &HSTRING) -> Result<()> {
        Ok(())
    }

    fn ObjectProperty(&self) -> Result<IInspectable> {
        Ok(self.to_interface())
    }

    fn SetObjectProperty(&self, _value: Ref<IInspectable>) -> Result<()> {
        Ok(())
    }

    fn ReferenceProperty(&self) -> Result<windows_reference::IReference<i32>> {
        Ok(windows_reference::IReference::<i32>::from(0))
    }

    fn SetReferenceProperty(&self, value: Ref<windows_reference::IReference<i32>>) -> Result<()> {
        if let Some(value) = value.as_ref() {
            _ = value.Value()?;
        }
        Ok(())
    }

    fn Operation(&self) -> Result<windows_future::IAsyncOperation<i32>> {
        Ok(windows_future::IAsyncOperation::<i32>::ready(Ok(0)))
    }

    fn StringOperation(&self) -> Result<windows_future::IAsyncOperation<HSTRING>> {
        Ok(windows_future::IAsyncOperation::<HSTRING>::ready(Ok(h!(
            "async"
        )
        .to_owned())))
    }

    fn ObjectOperation(&self) -> Result<windows_future::IAsyncOperation<bindings::INonDefault>> {
        Ok(
            windows_future::IAsyncOperation::<bindings::INonDefault>::ready(
                Ok(self.to_interface()),
            ),
        )
    }

    fn Add(&self, a: i32, b: i32) -> Result<i32> {
        Ok(a.wrapping_add(b))
    }

    fn SumArray(&self, values: &[i32]) -> Result<i32> {
        Ok(values.iter().copied().sum())
    }

    fn Values(&self) -> Result<Array<i32>> {
        Ok(Array::from_slice(&[1, 2, 3]))
    }

    fn GetValues(&self, values: &mut Array<i32>) -> Result<()> {
        *values = Array::from_slice(&[4, 5, 6]);
        Ok(())
    }

    fn EchoString(&self, value: &HSTRING) -> Result<HSTRING> {
        // Round-trip a string both ways: read the borrowed [in] HSTRING and return a +1 HSTRING the
        // caller owns. Appending a marker proves the input arrived intact and the output marshals
        // back, exercising a projection's HSTRING parameter and return path (not just properties).
        Ok(HSTRING::from(format!("{}-echo", value.to_string_lossy())))
    }

    fn Echo(&self, value: Ref<bindings::INonDefault>) -> Result<bindings::INonDefault> {
        // Borrow the incoming interface pointer and hand back a +1 reference the caller owns and
        // releases. This exercises object round-trip reference-count traffic: an [in] object is
        // borrowed (the caller keeps its own reference), and a returned object carries a new
        // reference the projection releases when it drops.
        Ok(value.ok()?.clone())
    }

    fn LiveCount(&self) -> Result<i32> {
        Ok(LIVE.load(std::sync::atomic::Ordering::Relaxed))
    }

    fn Fail(&self) -> Result<()> {
        // Always fails, so consumers can measure and validate HRESULT-to-error propagation.
        Err(HRESULT(0x8000_000B_u32 as i32).into()) // E_BOUNDS
    }

    fn FailWithMessage(&self) -> Result<()> {
        Err(Error::new(
            HRESULT(0x8000_000B_u32 as i32),
            "bench error detail",
        ))
    }

    fn Signal(&self, value: i32) -> Result<()> {
        // Invoke every registered handler, calling from native into the subscriber's delegate. For
        // a projected delegate this crosses the reverse vtable: native Invoke -> managed callback.
        let handlers = self.handlers.lock().unwrap();
        let sender: IInspectable = self.to_interface();
        for (_, handler) in handlers.iter() {
            handler.Invoke(&sender, value)?;
        }
        Ok(())
    }

    fn Items(&self, count: u32) -> Result<windows_collections::IVector<i32>> {
        // Return a vector of `count` sequential integers so a consumer can measure the per-element
        // cost of reading a projected generic collection across the ABI (GetAt/get_Size/GetMany).
        Ok(windows_collections::IVector::<i32>::from(
            (0..count as i32).collect::<Vec<i32>>(),
        ))
    }

    fn StringItems(&self, count: u32) -> Result<windows_collections::IVector<HSTRING>> {
        Ok(windows_collections::IVector::<HSTRING>::from(
            (0..count)
                .map(|i| HSTRING::from(i.to_string()))
                .collect::<Vec<_>>(),
        ))
    }

    fn Map(&self, count: u32) -> Result<windows_collections::IMap<i32, i32>> {
        // Return a map of `count` entries {i: i} so a consumer can measure the per-lookup cost of a
        // projected generic dictionary across the ABI (Lookup/get_Size/HasKey).
        Ok(windows_collections::IMap::<i32, i32>::from(
            (0..count as i32)
                .map(|i| (i, i))
                .collect::<std::collections::BTreeMap<i32, i32>>(),
        ))
    }

    fn StringMap(&self, count: u32) -> Result<windows_collections::IMap<HSTRING, i32>> {
        Ok(windows_collections::IMap::<HSTRING, i32>::from(
            (0..count as i32)
                .map(|i| (i.to_string().into(), i))
                .collect::<std::collections::BTreeMap<HSTRING, i32>>(),
        ))
    }

    fn StringValues(&self, count: u32) -> Result<windows_collections::IMap<i32, HSTRING>> {
        Ok(windows_collections::IMap::<i32, HSTRING>::from(
            (0..count as i32)
                .map(|i| (i, HSTRING::from(i.to_string())))
                .collect::<std::collections::BTreeMap<i32, HSTRING>>(),
        ))
    }

    fn ItemsView(&self, count: u32) -> Result<windows_collections::IVectorView<i32>> {
        // Return the read-only view of the same vector so a consumer can measure the per-element
        // cost of reading a projected `IVectorView<T>` (GetAt/get_Size/GetMany at the view's slots).
        windows_collections::IVector::<i32>::from((0..count as i32).collect::<Vec<i32>>()).GetView()
    }

    fn MapView(&self, count: u32) -> Result<windows_collections::IMapView<i32, i32>> {
        // Return the read-only view of the same map so a consumer can measure the per-lookup cost of
        // a projected `IMapView<K,V>` (Lookup/get_Size/HasKey at the view's slots).
        windows_collections::IMap::<i32, i32>::from(
            (0..count as i32)
                .map(|i| (i, i))
                .collect::<std::collections::BTreeMap<i32, i32>>(),
        )
        .GetView()
    }

    fn Changed(&self, handler: Ref<bindings::ChangedHandler>) -> Result<i64> {
        let token = self
            .next_token
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.handlers
            .lock()
            .unwrap()
            .push((token, handler.ok()?.clone()));
        Ok(token)
    }

    fn RemoveChanged(&self, token: i64) -> Result<()> {
        self.handlers.lock().unwrap().retain(|(t, _)| *t != token);
        Ok(())
    }
}

impl bindings::INonDefault_Impl for Widget_Impl {
    fn Value(&self) -> Result<i32> {
        Ok(self.value.load(std::sync::atomic::Ordering::Relaxed))
    }
}
