#ifndef NAMES
#define NAMES First, Second
#endif
#ifdef ENUM
typedef enum Tag { Value = 1 } NAMES;
#else
typedef struct Tag { int value; } NAMES;
#endif
