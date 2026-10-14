#ifndef TAG
#define TAG Bits
#endif
#ifndef WIDTH
#define WIDTH 3
#endif

typedef struct TAG {
    unsigned int value : WIDTH;
} PublicBits;
