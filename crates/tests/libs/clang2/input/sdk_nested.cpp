#include <stddef.h>
#include "sdk_nested.h"

static_assert(sizeof(DECIMAL) == 16 && alignof(DECIMAL) == 8);
static_assert(offsetof(DECIMAL, wReserved) == 0);
static_assert(offsetof(DECIMAL, scale) == 2);
static_assert(offsetof(DECIMAL, sign) == 3);
static_assert(offsetof(DECIMAL, signscale) == 2);
static_assert(offsetof(DECIMAL, Hi32) == 4);
static_assert(offsetof(DECIMAL, Lo32) == 8);
static_assert(offsetof(DECIMAL, Mid32) == 12);
static_assert(offsetof(DECIMAL, Lo64) == 8);
