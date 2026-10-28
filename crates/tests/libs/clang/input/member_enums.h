#ifndef HIDDEN_VALUE
#define HIDDEN_VALUE 99
#endif
struct Palette {
    enum { Blue = 0xFF0000FFu, AliceBlue = 0xFFF0F8FFu };
    enum { Shift = 24 };
    enum { Width = sizeof(void*) };
    unsigned int channel;
};
struct Channels {
    enum Channel : unsigned int { Red = 1, Green = 2 };
    enum class Mode : unsigned short { Normal = 0, Other = 1 };
    Channel channel;
};
class Color {
    unsigned int value;
    enum { Hidden = HIDDEN_VALUE };
public:
    struct Value { int value; };
    Color(unsigned int input) : value(input) {}
    enum { Red = 0xFFFF0000u, Blue = 0xFF0000FFu };
};
extern "C" void UsePalette(Palette* value);
extern "C" void UseChannel(Channels::Channel value);
