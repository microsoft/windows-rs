//! namespace ForwardDataClass
//! library api.dll
class Data;
typedef Data* PData;
class Data { public: unsigned int value; };
extern "C" void UseData(Data* value);
