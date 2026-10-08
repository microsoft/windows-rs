template <typename T> struct Dependent { typename T::member value; };
#define Bad Dependent<int>{}
#define Good 17
