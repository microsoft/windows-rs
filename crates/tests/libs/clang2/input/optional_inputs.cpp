#include "optional_inputs.h"

long long OptionalInteger(long long value) { return value; }
OptionalWord OptionalUnsigned(OptionalWord value) { return value; }
OptionalSigned OptionalSignedWord(OptionalSigned value) { return value; }
OptionalKind OptionalEnum(OptionalKind value) { return value; }
float OptionalFloat(float value) { return value; }
long OptionalRecord(OptionalPair value) { return value.value; }
long long OptionalPointer(const long long* value) { return value ? *value : -93; }
long long OptionalInvoke(OptionalCallback callback, long long value) {
    return callback ? callback(value) : -97;
}
