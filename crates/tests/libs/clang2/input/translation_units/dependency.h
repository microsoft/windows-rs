#pragma once

#ifdef FORWARD_ONLY
struct Shared;
#else
struct Shared {
#ifdef WIDE_SHARED
    long long value;
#else
    int value;
#endif
};
#endif

typedef unsigned int Count;
