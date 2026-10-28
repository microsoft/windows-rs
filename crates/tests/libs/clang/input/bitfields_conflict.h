#ifndef TAG
#define TAG Bits
#endif
#ifndef WIDTH
#define WIDTH 3
#endif
#ifndef MEMBER_TYPE
#define MEMBER_TYPE unsigned int
#endif

typedef struct TAG {
    MEMBER_TYPE value : WIDTH;
} PublicBits;
