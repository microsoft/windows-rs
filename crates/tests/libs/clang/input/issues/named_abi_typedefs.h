typedef unsigned __int64 HNSTIME;
typedef unsigned long mdToken;
typedef const unsigned char* PCCOR_SIGNATURE;
typedef unsigned long SCRIPTTHREADID;
struct Info {
    HNSTIME time;
    mdToken token;
    PCCOR_SIGNATURE signature;
    SCRIPTTHREADID thread;
};
