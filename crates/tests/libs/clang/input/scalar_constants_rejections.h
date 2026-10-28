extern "C" double RuntimeValue();
const double SIDE_EFFECT = (RuntimeValue(), 1.0);
const float INFINITE = __builtin_inff();
const double NOT_A_NUMBER = __builtin_nan("");
const long double EXTENDED = 1.25L;
double MUTABLE = 2.0;
const float SAFE = 1.0f;
