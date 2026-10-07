#pragma once

typedef struct NativeId {
    unsigned long first;
    unsigned short second;
    unsigned short third;
    unsigned char bytes[8];
} IdAlias;

struct NativeKey {
    IdAlias format;
    unsigned long property;
};

#ifndef LAST_BYTE
#define LAST_BYTE 0xfe
#endif

#ifdef DEFINE_VALUES
#define MAKE_ID(name) extern const IdAlias name = { 0xfedcba98, 0xabcd, 0x8765, { 0x80, 0xff, 0, 1, 2, 3, 4, LAST_BYTE } }
#define MAKE_KEY(name) extern const NativeKey name = { { 0xfedcba98, 0xabcd, 0x8765, { 0x80, 0xff, 0, 1, 2, 3, 4, LAST_BYTE } }, (1UL << 16) | 7 }
#else
#define MAKE_ID(name) extern const IdAlias name
#define MAKE_KEY(name) extern const NativeKey name
#endif

MAKE_ID(ID);
MAKE_KEY(KEY);
