#ifndef TAG
#define TAG First
#endif
#ifndef KIND
#define KIND union
#endif
#ifndef FIELD_TYPE
#define FIELD_TYPE int
#endif
KIND TAG { FIELD_TYPE value; };
typedef TAG Public;
