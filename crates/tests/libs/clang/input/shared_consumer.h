#include "shared_owner_a.h"
struct Consumer {
    SharedRecord record;
    SharedState state;
    SharedCallback callback;
};
