#pragma once
#include "shared.h"
#include "other/main.h"
#define HEADER_MARKER /* no replacement tokens */
#define HEADER_VALUE 7
#define HEADER_HELPER(value) ((value) + 1)
#define CURRENT_MODE Mode::Active
#define CURRENT_SIGNED SignedMode::Invalid
DECLARE_RECORD(Owned)
typedef Shared PublicShared;
struct Consumer { Shared value; Completed* other; };
struct Missing;
extern const int NoInitializer;
inline int InlineHelper() { return 0; }
