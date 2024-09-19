mod bindings;
use jsonschema::Validator;
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

// The `JsonValidator` struct represents the implementation of the `JsonValidator` class.
// The `implement` attribute provides the boilerplate COM and WinRT implementation support.
#[implement(bindings::JsonValidator)]
struct JsonValidator {
    schema: Validator,
}

// Implement the `IJsonValidator` interface.
impl bindings::IJsonValidator_Impl for JsonValidator_Impl {
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
impl IActivationFactory_Impl for JsonValidatorFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Err(E_NOTIMPL.into())
    }
}

// Implement the `IJsonValidatorFactory` interface.
impl bindings::IJsonValidatorFactory_Impl for JsonValidatorFactory_Impl {
    fn CreateInstance(&self, schema: &HSTRING) -> Result<bindings::JsonValidator> {
        let schema = json_from_hstring(schema)?;

        let schema =
            Validator::new(&schema).map_err(|error| Error::new(E_INVALIDARG, error.to_string()))?;

        Ok(JsonValidator { schema }.into())
    }
}

// Takes care of all the JSON parsing and parameter validation.
fn json_from_hstring(value: &HSTRING) -> Result<serde_json::Value> {
    let value = String::try_from(value)?;

    serde_json::from_str(&value).map_err(|error| Error::new(E_INVALIDARG, format!("{error}")))
}

static JSON_VALIDATOR_FACTORY: StaticComObject<JsonValidatorFactory> =
    JsonValidatorFactory.into_static();

#[no_mangle]
extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "Sample.JsonValidator" {
        factory
            .write(Some(JSON_VALIDATOR_FACTORY.to_interface()))
            .into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}
