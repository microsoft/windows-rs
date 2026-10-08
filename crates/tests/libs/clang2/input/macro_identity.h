struct Name {
    int value;
};

extern const int Name;
extern const int External;
const int Initialized = 17;

extern "C" void Use(struct Name* value);

#define Name (Name)
#define External (External)
#define Initialized (Initialized)
