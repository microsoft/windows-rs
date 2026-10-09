#include <windows.h>
#include "guid_expressions.h"
#include <ks.h>
#include <ksmedia.h>
#include <codecapi.h>
#include <cstring>

extern "C" int CheckGuidExpressions(
    const void* type, const void* pointer, const void* parens, const void* chain,
    const void* literal, const void* copied, const void* alias) {
    return std::memcmp(type, &TYPE_ID, sizeof(GUID)) == 0
        && std::memcmp(pointer, &POINTER_ID, sizeof(GUID)) == 0
        && std::memcmp(parens, &PAREN_ID, sizeof(GUID)) == 0
        && std::memcmp(chain, &CHAIN_ID, sizeof(GUID)) == 0
        && std::memcmp(literal, &LiteralId, sizeof(GUID)) == 0
        && std::memcmp(copied, &CopiedId, sizeof(GUID)) == 0
        && std::memcmp(alias, &LITERAL_ALIAS, sizeof(GUID)) == 0;
}

extern "C" int CheckSdkExpressionValues(
    const void* ks, const void* media, const void* codec) {
    return std::memcmp(ks, &KSPROPSETID_General, sizeof(GUID)) == 0
        && std::memcmp(media, &KSMEDIUMSETID_MidiBus, sizeof(GUID)) == 0
        && std::memcmp(codec, &CODECAPI_AVEncCommonFormatConstraint, sizeof(GUID)) == 0;
}
