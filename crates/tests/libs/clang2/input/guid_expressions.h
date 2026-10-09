#include <guiddef.h>

#ifndef EXPRESSION_UUID
#define EXPRESSION_UUID "fedcba98-abcd-8765-80ff-0001020304fe"
#endif
struct __declspec(uuid(EXPRESSION_UUID)) IdentityOwner;
using IdentityAlias = IdentityOwner;

#define TYPE_ID __uuidof(IdentityAlias)
#define POINTER_ID __uuidof(IdentityOwner*)
#define PAREN_ID ((__uuidof(IdentityOwner)))
#define CHAIN_ID TYPE_ID

extern const GUID LiteralId = {
    0xfedcba98, 0xabcd, 0x8765, {0x80, 0xff, 0, 1, 2, 3, 4, 0xfe}
};
extern const GUID CopiedId = LiteralId;
#define LITERAL_ALIAS CopiedId
