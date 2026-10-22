#pragma once
#define REQUESTED_ALIGNMENT 16
struct alignas(REQUESTED_ALIGNMENT) Standard { char value; };
struct __declspec(align(32)) Declspec { char value; };
struct __attribute__((aligned(64))) Gnu { char value; };
struct alignas(Standard) TypeAligned { char value; };
struct alignas(8) alignas(16) Multiple { char value; };
struct alignas(0) Zero { char value; };
struct __declspec(align(4)) Unchanged { long long value; };
union __declspec(align(32)) AlignedUnion { int value; char bytes[3]; };
struct Container { char lead; Standard nested; Standard array[2]; char tail; };
struct alignas(16) Recursive { Recursive* next; };
#if defined(_M_IX86)
struct alignas(16) Platform { char value; };
#else
struct alignas(32) Platform { char value; };
#endif
