enum class Kind : unsigned short { First = 1, Last = 65535 };
extern "C" Kind Convert(Kind value);
struct Packet { Kind kind; };
