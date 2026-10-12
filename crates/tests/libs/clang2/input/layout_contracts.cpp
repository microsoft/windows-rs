#include <stddef.h>

#pragma warning(push)
#pragma warning(disable: 4121 4324 4359)
#include "layout_contracts.h"
#pragma warning(pop)

static_assert(sizeof(PackOne) == 5 && alignof(PackOne) == 1);
static_assert(offsetof(PackOne, tag) == 0 && offsetof(PackOne, value) == 1);
static_assert(sizeof(PackTwo) == 4 && alignof(PackTwo) == 2);
static_assert(offsetof(PackTwo, tag) == 0 && offsetof(PackTwo, value) == 2);
static_assert(sizeof(PackFour) == 4 && alignof(PackFour) == 2);
static_assert(offsetof(PackFour, tag) == 0 && offsetof(PackFour, value) == 2);
static_assert(sizeof(Aligned) == 16 && alignof(Aligned) == 16);
static_assert(offsetof(Aligned, value) == 0);
static_assert(sizeof(AlignOne) == 4 && alignof(AlignOne) == 4);
static_assert(sizeof(AlignFour) == 4 && alignof(AlignFour) == 4);
static_assert(sizeof(FieldAligned) == 32 && alignof(FieldAligned) == 16);
static_assert(offsetof(FieldAligned, tag) == 0 && offsetof(FieldAligned, value) == 16);
static_assert(sizeof(PackedPointer) == 2 + sizeof(void *) && alignof(PackedPointer) == 2);
static_assert(offsetof(PackedPointer, tag) == 0 && offsetof(PackedPointer, value) == 2);
static_assert(sizeof(PackedUnion) == 8 && alignof(PackedUnion) == 2);
static_assert(offsetof(PackedUnion, code) == 0 && offsetof(PackedUnion, value) == 0);
static_assert(sizeof(PackedContainer) == 64 && alignof(PackedContainer) == 16);
static_assert(offsetof(PackedContainer, tag) == 0);
static_assert(offsetof(PackedContainer, values) == 16);
static_assert(offsetof(PackedContainer, bytes) == 48);
static_assert(sizeof(PackedOuter) == 80 && alignof(PackedOuter) == 16);
static_assert(offsetof(PackedOuter, tag) == 0 && offsetof(PackedOuter, value) == 16);
