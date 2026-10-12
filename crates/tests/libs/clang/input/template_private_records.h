//! namespace Test
//! library test.dll
template <class T, unsigned Magic>
class ItemFactory {
private:
    typedef struct {
        unsigned marker;
        T innerData;
    } PRIVATE_ITEM;
    typedef PRIVATE_ITEM* PRIVATE_PTR;
};

template <class T>
class PartialFactory;

template <class T>
class PartialFactory<T*> {
private:
    typedef struct {
        unsigned marker;
        T innerData;
    } PRIVATE_PARTIAL;
};

typedef struct {
    int payload;
} GLOBAL_ITEM;
typedef GLOBAL_ITEM* GLOBAL_PTR;
struct NODE {
    NODE* next;
};
extern "C" void Keep(GLOBAL_ITEM* value, NODE* node);
