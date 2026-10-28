#pragma once

struct Leaf {
    int value;
};
struct Required;
struct Opaque;
extern "C" {
    struct Required {
        Leaf values[2];
        Required* next;
    };
}
struct Required;

typedef int UnusedAlias;
union UnusedUnion {
    int integer;
    double real;
};
struct Unused {
    unsigned flag : 1;
};
int unused_function(UnusedAlias value);
namespace Ignored {
    struct Hidden { int value; };
}
