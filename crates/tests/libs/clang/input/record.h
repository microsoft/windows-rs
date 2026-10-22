struct Record {
    int count;
    long native_long;
    const struct Record* next;
    unsigned char bytes[3];
};
