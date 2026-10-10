template<int> struct Gate;
template<> struct Gate<1> { static const int value = 23; };
#define ACounter __COUNTER__
#define ZGood Gate<__COUNTER__>::value
