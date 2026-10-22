#pragma once
#define DECLARE_TAG(name) struct name##__ { int value; }; typedef struct name##__* name
#define WRAP_TAG(name) DECLARE_TAG(name)
