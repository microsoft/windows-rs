#include "optional_output_counts.h"

static unsigned fill(unsigned char* buffer, unsigned capacity) {
    unsigned count = buffer ? (capacity < 3 ? capacity : 3) : 0;
    for (unsigned index = 0; index < count; ++index) {
        buffer[index] = static_cast<unsigned char>(index + 1);
    }
    return count;
}

extern "C" void FillRequired(unsigned char* buffer, unsigned capacity, unsigned* written) {
    unsigned count = fill(buffer, capacity);
    if (written) {
        *written = count;
    }
}

extern "C" void FillOptional(unsigned char* buffer, unsigned capacity, unsigned* written) {
    FillRequired(buffer, capacity, written);
}

extern "C" void FillInout(unsigned char* buffer, unsigned capacity, unsigned* written) {
    if (written && *written < capacity) {
        capacity = *written;
    }
    FillRequired(buffer, capacity, written);
}

extern "C" void InvokeOptional(
    OptionalCallback callback, unsigned char* buffer, unsigned capacity, unsigned* written) {
    callback(buffer, capacity, written);
}
