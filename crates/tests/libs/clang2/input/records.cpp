#include "records.h"
#include <stddef.h>

namespace Anchor {
#include "../../clang/input/data_classes.h"
}

static_assert(offsetof(Anchor::BitmapData, width) == 0);
static_assert(offsetof(Anchor::BitmapData, stride) == 4);
static_assert(offsetof(Anchor::BitmapData, scan) == 8);
static_assert(sizeof(Point) == 8 && alignof(Point) == 4);
static_assert(offsetof(Point, x) == 0 && offsetof(Point, y) == 4);
static_assert(sizeof(Storage) == 8 && alignof(Storage) == 8);
static_assert(offsetof(Storage, wide) == 0 && offsetof(Storage, bytes) == 0);
static_assert(sizeof(Arrays) == 20 && alignof(Arrays) == 4);
static_assert(offsetof(Arrays, points) == 0 && offsetof(Arrays, state) == 16);
static_assert(sizeof(Left) == sizeof(void *) && alignof(Left) == alignof(void *));
static_assert(sizeof(Right) == sizeof(void *) && alignof(Right) == alignof(void *));
static_assert(offsetof(Left, right) == 0 && offsetof(Right, left) == 0);
static_assert(offsetof(Node, next) == 0);
static_assert(offsetof(Container, tag) == 0);
static_assert(offsetof(Container, point) == 4);
static_assert(offsetof(Container, storage) == 16);
static_assert(offsetof(Container, node) == 24);

#ifdef _M_IX86
static_assert(sizeof(Anchor::BitmapData) == 12 && alignof(Anchor::BitmapData) == 4);
static_assert(sizeof(Node) == 8 && alignof(Node) == 4);
static_assert(offsetof(Node, value) == 4);
static_assert(sizeof(Container) == 32 && alignof(Container) == 8);
static_assert(offsetof(Container, value) == 28);
#elif defined(_M_X64) || defined(_M_ARM64)
static_assert(sizeof(Anchor::BitmapData) == 16 && alignof(Anchor::BitmapData) == 8);
static_assert(sizeof(Node) == 16 && alignof(Node) == 8);
static_assert(offsetof(Node, value) == 8);
static_assert(sizeof(Container) == 40 && alignof(Container) == 8);
static_assert(offsetof(Container, value) == 32);
#else
#error Unsupported record evidence target
#endif
