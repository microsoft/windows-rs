#include <stdint.h>
#include <assert.h>
#include <windows.h>
#include <string_view>

typedef HRESULT (__stdcall *CreateJsonValidator)(char const* schema, size_t schema_len, uintptr_t* handle);

typedef HRESULT (__stdcall *ValidateJson)(uintptr_t handle, char const* value, size_t value_len, char** sanitized_value, size_t* sanitized_value_len);

typedef void (__stdcall *CloseJsonValidator)(uintptr_t handle);

extern "C" {
    void __stdcall client() {
        auto library = LoadLibraryExW(L"sample_json_validator.dll", 0, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);
        assert(library != 0);

        auto create = reinterpret_cast<CreateJsonValidator>(GetProcAddress(library, "CreateJsonValidator"));
        assert(create);

        auto validate = reinterpret_cast<ValidateJson>(GetProcAddress(library, "ValidateJson"));
        assert(validate);

        auto close = reinterpret_cast<CloseJsonValidator>(GetProcAddress(library, "CloseJsonValidator"));
        assert(close);

        std::string_view schema = "{\"maxLength\": 5}";
        std::string_view json = "\"Hello\" "; // trailing space will be removed from sanitized result
        std::string_view json_invalid = "\"Hello world\""; // this json is too long

        uintptr_t validator = 0;
        assert(S_OK == create(schema.data(), schema.size(), &validator));

        char* sanitized_value = nullptr;
        size_t sanitized_value_len = 0;
        assert(S_OK == validate(validator, json.data(), json.size(), &sanitized_value, &sanitized_value_len));
        std::string_view sanitized(sanitized_value, sanitized_value_len);
        assert(sanitized == "\"Hello\"");
        CoTaskMemFree(sanitized_value);

        assert(E_INVALIDARG == validate(validator, json_invalid.data(), json_invalid.size(), &sanitized_value, &sanitized_value_len));

        close(validator);
    }
}
