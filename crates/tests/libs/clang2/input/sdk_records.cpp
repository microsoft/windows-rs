#include "winnt.h"
#include <stddef.h>

static_assert(sizeof(POINT) == 8 && alignof(POINT) == 4);
static_assert(offsetof(POINT, x) == 0 && offsetof(POINT, y) == 4);
static_assert(sizeof(RECT) == 16 && alignof(RECT) == 4);
static_assert(offsetof(RECT, left) == 0 && offsetof(RECT, top) == 4);
static_assert(offsetof(RECT, right) == 8 && offsetof(RECT, bottom) == 12);
