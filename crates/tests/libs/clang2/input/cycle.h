struct First;
struct Second;
struct First { Second* value; };
struct Second { First* value; VALUE leaf; };
typedef First* Raw;
typedef Raw Alias;
extern "C" void Use(Alias value);
