#pragma once
#include "dependency.h"

struct Packet {
    Shared* shared;
    Count count;
};

#ifndef DIRECTION
#define DIRECTION "_In_"
#endif

extern "C" void Use(Packet* __attribute__((annotate(DIRECTION))) packet, Count count);
