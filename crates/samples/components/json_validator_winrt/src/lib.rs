mod bindings;
use jsonschema::JSONSchema;
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

// The `JsonValidator` struct represents the implementation of the `JsonValidator` class.
// The `implement` attribute provides the boilerplate COM and WinRT implementation support.
#[implement(bindings::JsonValidator)]
struct JsonValidator {
    schema: JSONSchema,
}

// Implement the `IJsonValidator` interface.
impl bindings::IJsonValidator_Impl for JsonValidator {
    fn Validate(&self, value: &HSTRING) -> Result<HSTRING> {
        let value = json_from_hstring(value)?;

        if self.schema.is_valid(&value) {
            // Return the sanitized value.
            Ok(value.to_string().into())
        } else {
            // The `validate` method returns a collection of errors. We'll just return the first
            // for simplicity.
            let message = self
                .schema
                .validate(&value)
                .unwrap_err()
                .next()
                .map_or(String::new(), |error| error.to_string());

            Err(Error::new(E_INVALIDARG, message))
        }
    }
}

// The `JsonValidatorFactory` struct represents the implementation of the `JsonValidator` class factory.
#[implement(IActivationFactory, bindings::IJsonValidatorFactory)]
struct JsonValidatorFactory;

// The JsonValidator class doesn't provide a default constructor but WinRT still requires an
// implementation of `IActivationFactory`.
impl IActivationFactory_Impl for JsonValidatorFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Err(E_NOTIMPL.into())
    }
}

// Implement the `IJsonValidatorFactory` interface.
impl bindings::IJsonValidatorFactory_Impl for JsonValidatorFactory {
    fn CreateInstance(&self, schema: &HSTRING) -> Result<bindings::JsonValidator> {
        let schema = json_from_hstring(schema)?;

        let schema = JSONSchema::compile(&schema)
            .map_err(|error| Error::new(E_INVALIDARG, error.to_string()))?;

        Ok(JsonValidator { schema }.into())
    }
}

// Takes care of all the JSON parsing and parameter validation.
fn json_from_hstring(value: &HSTRING) -> Result<serde_json::Value> {
    let value = String::try_from(value)?;

    serde_json::from_str(&value).map_err(|error| Error::new(E_INVALIDARG, format!("{error}")))
}

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    if result.is_null() {
        return E_POINTER;
    }

    let mut factory: Option<IActivationFactory> = None;

    if *name == "Sample.JsonValidator" {
        factory = Some(JsonValidatorFactory.into());
    }

    // Dereferencing `result` is safe because we've validated that the pointer is not null and
    // transmuting `factory` is safe because `DllGetActivationFactory` is expected to return an
    // `IActivationFactory` pointer and that's what `factory` contains.
    unsafe {
        if let Some(factory) = factory {
            *result = factory.into_raw();
            S_OK
        } else {
            *result = std::ptr::null_mut();
            CLASS_E_CLASSNOTAVAILABLE
        }
    }
}
