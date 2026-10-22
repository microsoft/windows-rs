#include <intsafe.h>
#include <windows.h>
typedef void (*SegmentConsumer)(FILE_SEGMENT_ELEMENT segments[]);
static_assert(sizeof(FILE_SEGMENT_ELEMENT) == 8);
static_assert(alignof(FILE_SEGMENT_ELEMENT) == 8);
static_assert(__builtin_offsetof(FILE_SEGMENT_ELEMENT, Buffer) == 0);
static_assert(sizeof(PVOID64) == 8);
