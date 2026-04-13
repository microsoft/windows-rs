#pragma once
/* Minimal shim for <EventToken.h> — defines EventRegistrationToken. */

/* EventRegistrationToken is a WinRT struct wrapping a 64-bit value. */
typedef struct EventRegistrationToken {
    long long value;
} EventRegistrationToken;
