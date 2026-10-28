struct Alpha;
struct Beta;
typedef Alpha* Handle;
struct Holder {
    Handle first;
    const Alpha* second;
    Beta* third;
};
extern "C" Handle Echo(Handle value);
#define INVALID ((Handle)-1)
