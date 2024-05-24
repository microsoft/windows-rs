#include "winrt/Sample.h"

using namespace winrt::Sample;

extern "C" {
    void __stdcall client() noexcept {
        auto schema = LR"(
{
    "properties": {
        "name": {
            "type": "string"
        },
        "age": {
            "type": "integer"
        }
    }
}
    )";

        auto value = LR"(
{
    "name": "Kenny",
    "age": 21 
}
    )";

        // Create a validator with the given schema.
        auto validator = JsonValidator(schema);

        // Validate and check the sanitized return value.
        auto sanitized = validator.Validate(value);
        assert(sanitized == LR"({"age":21,"name":"Kenny"})");
    }
}
