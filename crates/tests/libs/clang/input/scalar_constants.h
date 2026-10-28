typedef float Scale;
constexpr float twice(float value) { return value * 2.0f; }
const bool ENABLED = true;
const bool DISABLED = false;
const Scale FRACTION = 0.1f;
const double PRECISE = 0.1;
const float NEGATIVE_ZERO = -0.0f;
const double NEGATIVE_ZERO64 = -0.0;
const float ROUNDED = 16777217.0f;
const float SMALLEST = 0x1p-149f;
const double LARGEST = 0x1.fffffffffffffp1023;
const float FOLDED = twice(0.125f);
const double NATIVE_WIDTH = sizeof(void*);
#define MACRO_SCALE (0.5f + 0.25f)
#define MACRO_FLAG (sizeof(void*) == 8)
