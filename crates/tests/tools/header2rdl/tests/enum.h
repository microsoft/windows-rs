typedef enum _STATUS {
    STATUS_IDLE = 0,
    STATUS_RUNNING = 1,
    STATUS_ERROR = -1,
} STATUS;

typedef enum _FLAGS {
    FLAG_NONE = 0,
    FLAG_READ = 1,
    FLAG_WRITE = 2,
    FLAG_EXECUTE = 4,
} FLAGS;
