#include <stddef.h>

#pragma warning(push)
#pragma warning(disable: 4121 4201)
#include "nested_records.h"
#pragma warning(pop)

static_assert(sizeof(Shapes::Owner::Node) == (sizeof(void *) == 4 ? 8 : 16));
static_assert(alignof(Shapes::Owner::Node) == sizeof(void *));
static_assert(offsetof(Shapes::Owner::Node, value) == 0);
static_assert(offsetof(Shapes::Owner::Node, next) == sizeof(void *));
static_assert(sizeof(Shapes::Peer::Node) == sizeof(Shapes::Owner::Node));
static_assert(sizeof(Shapes::Peer) == sizeof(Shapes::Peer::Node));
static_assert(offsetof(Shapes::Peer, item) == 0);
static_assert(sizeof(Shapes::Owner::Payload) == 8 && alignof(Shapes::Owner::Payload) == 8);
static_assert(offsetof(Shapes::Owner::Payload, code) == 0);
static_assert(offsetof(Shapes::Owner::Payload, value) == 0);
static_assert(sizeof(Shapes::Owner) == (sizeof(void *) == 4 ? 64 : 88));
static_assert(alignof(Shapes::Owner) == 8);
static_assert(offsetof(Shapes::Owner, tag) == 0);
static_assert(offsetof(Shapes::Owner, nodes) == (sizeof(void *) == 4 ? 4 : 8));
static_assert(offsetof(Shapes::Owner, link) == (sizeof(void *) == 4 ? 20 : 40));
static_assert(offsetof(Shapes::Owner, first) == (sizeof(void *) == 4 ? 24 : 48));
static_assert(offsetof(Shapes::Owner, second) == (sizeof(void *) == 4 ? 32 : 56));
static_assert(offsetof(Shapes::Owner, detail) == (sizeof(void *) == 4 ? 40 : 64));
static_assert(offsetof(Shapes::Owner, code) == (sizeof(void *) == 4 ? 48 : 72));
static_assert(offsetof(Shapes::Owner, low) == offsetof(Shapes::Owner, code));
static_assert(offsetof(Shapes::Owner, high) == offsetof(Shapes::Owner, low) + 2);
static_assert(offsetof(Shapes::Owner, scalar) == (sizeof(void *) == 4 ? 52 : 76));
static_assert(offsetof(Shapes::Owner, integer) == offsetof(Shapes::Owner, scalar));
static_assert(offsetof(Shapes::Owner, Anonymous_7) == (sizeof(void *) == 4 ? 56 : 80));
static_assert(sizeof(PackedNested::Inner) == 6 && alignof(PackedNested::Inner) == 2);
static_assert(offsetof(PackedNested::Inner, tag) == 0);
static_assert(offsetof(PackedNested::Inner, value) == 2);
static_assert(sizeof(PackedNested) == 10 && alignof(PackedNested) == 2);
static_assert(offsetof(PackedNested, item) == 0 && offsetof(PackedNested, number) == 6);
static_assert(offsetof(PackedNested, code) == 6);
