//! namespace Records
//! input forward.h
typedef struct SHARED SHARED;
typedef SHARED* PSHARED;
extern "C" void UseShared(SHARED value);
//! input complete.h
typedef struct SHARED { unsigned int value; } SHARED;
