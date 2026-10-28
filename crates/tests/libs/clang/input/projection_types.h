#pragma once

typedef unsigned long Count;

typedef struct _Point {
    long x;
    long y;
} Point;

typedef struct _Node {
    Count count;
    const struct _Node* next;
    Point points[2];
} Node;

typedef struct __attribute__((packed)) _Packed {
    char tag;
#if defined(_M_IX86)
    long value;
#else
    long long value;
#endif
} Packed;

struct Variant {
#if defined(_M_IX86)
    int value;
#elif defined(_M_X64)
    long long value;
#else
    void* value;
#endif
};
