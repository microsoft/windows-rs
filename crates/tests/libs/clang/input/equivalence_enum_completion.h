//! filter api.h
//! file api.h
enum class Flags : unsigned int;
typedef Flags Alias;
struct Uses { Alias value; };
extern "C" void Use(Uses value);
//! input first.h
#include "api.h"
//! input second.h
enum class Flags : unsigned int { One = 1 };
#include "api.h"
