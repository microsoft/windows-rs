#pragma once
#define DECLARE_RECORD(name) struct name { int value; };
#ifdef CONFLICT
struct Shared { long long value; };
#else
struct Shared { int value; };
#endif
struct Completed;
enum class Mode : unsigned short { Active = 3 };
enum class SignedMode : int { Invalid = -2 };
